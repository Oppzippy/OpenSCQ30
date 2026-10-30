use std::{
    collections::HashSet,
    ffi::{c_int, c_void},
    panic::Location,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use async_trait::async_trait;
use macaddr::MacAddr6;
use objc2::{AnyThread, DefinedClass, define_class, rc::Retained, runtime::AnyObject};
use objc2_core_foundation::{CFRunLoop, CFTimeInterval, kCFRunLoopDefaultMode};
use objc2_foundation::{NSArray, NSObject, NSObjectProtocol, NSString};
use objc2_io_bluetooth::{
    BluetoothRFCOMMChannelID, BluetoothSDPServiceAttributeID, IOBluetoothDevice,
    IOBluetoothRFCOMMChannel, IOBluetoothSDPDataElement, IOBluetoothSDPServiceRecord,
    IOBluetoothSDPUUID,
};
use tokio::sync::{mpsc, watch};
use tracing::{debug, debug_span, trace};
use uuid::Uuid;

use crate::{
    api::connection::{self, RfcommBackend, RfcommConnection},
    connection::RfcommServiceSelectionStrategy,
};

// Bluetooth SDP spec: the ServiceClassIDList attribute, which holds the UUIDs a service record advertises.
const SERVICE_CLASS_ID_LIST_ATTRIBUTE_ID: BluetoothSDPServiceAttributeID = 0x0001;

// A channel can briefly report "not open" right after connecting while IOBluetooth finishes the
// underlying L2CAP negotiation, even though openRFCOMMChannelAsync already returned success. How
// long this takes is inconsistent in practice (observed anywhere from ~200ms to over 2s), so the
// retry budget is generous rather than tight.
const IO_RETURN_NOT_OPEN: c_int = 0xE00002CDu32 as c_int;
const WRITE_NOT_OPEN_RETRIES: u32 = 50;

const SDP_QUERY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

// How long to wait for rfcommChannelOpenComplete after openRFCOMMChannelAsync returns success.
// Observed at ~0.5-1s on macOS 15 when it succeeds; when the channel is stuck it never arrives.
const RFCOMM_OPEN_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

// closeConnection is asynchronous: isConnected() stays true for a while afterwards (~0.5s
// observed on macOS 15), and calling openConnection/openRFCOMMChannelAsync during that window
// leaves the RFCOMM channel permanently "not open". Bounds how long to wait for it to settle.
const BASEBAND_CLOSE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
const BASEBAND_CLOSE_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);

// CFRunLoopRun() returns immediately if nothing is scheduled at the moment it's called, so the
// connection's event loop re-enters it in short bursts instead of once indefinitely.
const RFCOMM_EVENT_POLL_SECONDS: CFTimeInterval = 0.1;

// How long the connection thread waits for a write request before falling back to pumping the
// run loop; bounds how stale an unpumped inbound callback can get, so kept short.
const WRITE_DRAIN_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(10);

#[derive(Default)]
pub struct MacosRfcommBackend;

#[async_trait]
impl RfcommBackend for MacosRfcommBackend {
    async fn devices(&self) -> connection::Result<HashSet<connection::ConnectionDescriptor>> {
        tokio::task::spawn_blocking(|| {
            // Skip devices whose address can't be read rather than failing the whole list.
            Ok(paired_devices()
                .into_iter()
                .filter_map(|device| {
                    let mac_address = mac_address_of(&device).ok()?;
                    Some(connection::ConnectionDescriptor {
                        name: unsafe { device.nameOrAddress() }
                            .map(|name| name.to_string())
                            .unwrap_or_default(),
                        mac_address,
                    })
                })
                .collect())
        })
        .await
        .unwrap()
    }

    async fn connect(
        &self,
        mac_address: MacAddr6,
        service_selection_strategy: RfcommServiceSelectionStrategy,
    ) -> connection::Result<Arc<dyn RfcommConnection + Send + Sync>> {
        tokio::task::spawn_blocking(
            move || -> connection::Result<Arc<dyn RfcommConnection + Send + Sync>> {
                let span = debug_span!(
                    "RfcommBackend::connect",
                    mac_address = tracing::field::display(mac_address),
                );
                let _span_guard = span.enter();

                debug!("finding device with desired mac address");
                let device = device_with_mac_address(mac_address)?;

                let was_connected = unsafe { device.isConnected() };
                if !was_connected {
                    debug!("opening baseband connection");
                    open_baseband_connection(&device)?;
                }

                debug!("selecting RFCOMM service");
                let channel_id = select_channel_id(&device, &service_selection_strategy)?;

                debug!("opening RFCOMM channel");
                match MacosRfcommConnection::open(device.clone(), channel_id) {
                    Ok(connection) => Ok(Arc::new(connection)),
                    // The RFCOMM channel can get stuck "not open" forever regardless of whether
                    // the baseband connection was just opened or already existed. Cycling it is
                    // the only known workaround, but it audibly disconnects/reconnects the device
                    // (e.g. an audio chime) and costs a few seconds, so it's only done once the
                    // fast path has demonstrably failed rather than on every connect.
                    Err(connection::Error::TimedOut { .. }) => {
                        debug!("RFCOMM channel did not open, cycling baseband connection");
                        cycle_baseband_connection(&device)?;
                        debug!("retrying RFCOMM channel open");
                        Ok(Arc::new(MacosRfcommConnection::open(device, channel_id)?))
                    }
                    Err(err) => Err(err),
                }
            },
        )
        .await
        .unwrap()
    }
}

fn paired_devices() -> Vec<Retained<IOBluetoothDevice>> {
    unsafe { IOBluetoothDevice::pairedDevices() }
        .map(|devices| downcast_all::<IOBluetoothDevice>(&devices))
        .unwrap_or_default()
}

// pairedDevices()/services()/getArrayValue() are untyped NSArray<AnyObject>, so elements need a runtime downcast.
fn downcast_all<T: objc2::DowncastTarget>(array: &NSArray<AnyObject>) -> Vec<Retained<T>> {
    array
        .to_vec()
        .into_iter()
        .filter_map(|element| element.downcast::<T>().ok())
        .collect()
}

fn mac_address_of(device: &IOBluetoothDevice) -> connection::Result<MacAddr6> {
    let address_string =
        unsafe { device.addressString() }.ok_or_else(|| connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        })?;
    address_string
        .to_string()
        .replace('-', ":")
        .parse()
        .map_err(|err: macaddr::ParseError| connection::Error::Other {
            source: Box::new(err),
            location: Location::caller(),
        })
}

fn device_with_mac_address(
    mac_address: MacAddr6,
) -> connection::Result<Retained<IOBluetoothDevice>> {
    let address_string = NSString::from_str(&mac_address.to_string().replace(':', "-"));
    unsafe { IOBluetoothDevice::deviceWithAddressString(Some(&address_string)) }.ok_or(
        connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        },
    )
}

fn cycle_baseband_connection(device: &IOBluetoothDevice) -> connection::Result<()> {
    let close_status = unsafe { device.closeConnection() };
    if close_status != 0 {
        debug!("closeConnection returned status {close_status}, opening anyway");
    }
    let started_at = std::time::Instant::now();
    while unsafe { device.isConnected() } {
        if started_at.elapsed() >= BASEBAND_CLOSE_TIMEOUT {
            // Opening now would race the still-in-flight close: the exact precondition that
            // leaves the RFCOMM channel permanently stuck (see BASEBAND_CLOSE_TIMEOUT above).
            // Fail instead of guaranteeing that same failure.
            return Err(connection::Error::TimedOut {
                action: "close baseband connection",
            });
        }
        thread::sleep(BASEBAND_CLOSE_POLL_INTERVAL);
    }
    debug!(
        "baseband connection closed after {:?}",
        started_at.elapsed()
    );
    open_baseband_connection(device)
}

fn open_baseband_connection(device: &IOBluetoothDevice) -> connection::Result<()> {
    let status = unsafe { device.openConnection() };
    if status != 0 {
        return Err(connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        });
    }
    Ok(())
}

fn select_channel_id(
    device: &IOBluetoothDevice,
    service_selection_strategy: &RfcommServiceSelectionStrategy,
) -> connection::Result<BluetoothRFCOMMChannelID> {
    let uuid = match service_selection_strategy {
        RfcommServiceSelectionStrategy::Constant(uuid) => *uuid,
        RfcommServiceSelectionStrategy::Dynamic(select_uuid) => {
            let uuids = service_uuids(device);
            debug!("found RFCOMM services: {uuids:?}");
            let uuid = select_uuid(uuids);
            debug!("using RFCOMM service: {uuid}");
            uuid
        }
    };

    let record = service_record_for_uuid(device, uuid)?;
    let mut channel_id: BluetoothRFCOMMChannelID = 0;
    let status = unsafe { record.getRFCOMMChannelID(&mut channel_id) };
    if status != 0 {
        return Err(connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        });
    }
    debug!("resolved RFCOMM channel id: {channel_id}");
    Ok(channel_id)
}

fn service_record_for_uuid(
    device: &IOBluetoothDevice,
    uuid: Uuid,
) -> connection::Result<Retained<IOBluetoothSDPServiceRecord>> {
    let sdp_uuid = sdp_uuid_from_uuid(uuid).ok_or(connection::Error::DeviceNotFound {
        source: None,
        location: Location::caller(),
    })?;
    unsafe { device.getServiceRecordForUUID(Some(&sdp_uuid)) }.ok_or(
        connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        },
    )
}

// macOS only populates the SDP cache once System Settings' device info pane has been opened, so
// query if empty. Not forced on every connect: performSDPQuery can hang indefinitely when called
// repeatedly in quick succession.
fn service_uuids(device: &IOBluetoothDevice) -> HashSet<Uuid> {
    let mut records = cached_service_records(device);
    if records.is_empty() {
        debug!("no cached SDP records, performing SDP query");
        if let Err(err) = perform_sdp_query(device) {
            debug!("SDP query failed: {err}");
            return HashSet::new();
        }
        records = cached_service_records(device);
    }
    debug!("found {} cached SDP service records", records.len());
    records
        .into_iter()
        .flat_map(|record| service_class_uuids(&record))
        .collect()
}

fn cached_service_records(
    device: &IOBluetoothDevice,
) -> Vec<Retained<IOBluetoothSDPServiceRecord>> {
    unsafe { device.services() }
        .map(|records| downcast_all::<IOBluetoothSDPServiceRecord>(&records))
        .unwrap_or_default()
}

fn perform_sdp_query(device: &IOBluetoothDevice) -> connection::Result<()> {
    let (done_sender, done_receiver) = std::sync::mpsc::channel();
    let delegate = SdpQueryDelegate::new(done_sender);
    let delegate_object: &AnyObject = &delegate;
    let status = unsafe { device.performSDPQuery(Some(delegate_object)) };
    if status != 0 {
        return Err(connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        });
    }

    // sdpQueryComplete: is delivered on the process main thread (pumped continuously by
    // cli/src/main.rs's run loop), not this thread, so a run loop pumped here would never see
    // it; recv_timeout on the plain mpsc channel is what actually bounds the wait.
    match done_receiver.recv_timeout(SDP_QUERY_TIMEOUT) {
        Ok(0) => Ok(()),
        Ok(_) => Err(connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        }),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            // IOBluetoothDevice doesn't retain the target passed to performSDPQuery, and there's
            // no way to cancel it, so the query may still be in flight here. Dropping delegate
            // normally would risk sdpQueryComplete: firing into deallocated memory later; leak it
            // instead (bounded: at most once per timed-out query).
            std::mem::forget(delegate);
            Err(connection::Error::TimedOut {
                action: "SDP query",
            })
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            Err(connection::Error::DeviceNotFound {
                source: None,
                location: Location::caller(),
            })
        }
    }
}

struct SdpQueryState {
    done_sender: std::sync::mpsc::Sender<c_int>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = SdpQueryState]
    struct SdpQueryDelegate;

    unsafe impl NSObjectProtocol for SdpQueryDelegate {}

    impl SdpQueryDelegate {
        #[unsafe(method(sdpQueryComplete:status:))]
        fn sdp_query_complete(&self, _device: Option<&IOBluetoothDevice>, status: c_int) {
            let _ = self.ivars().done_sender.send(status);
        }
    }
);

impl SdpQueryDelegate {
    fn new(done_sender: std::sync::mpsc::Sender<c_int>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(SdpQueryState { done_sender });
        unsafe { objc2::msg_send![super(this), init] }
    }
}

fn service_class_uuids(record: &IOBluetoothSDPServiceRecord) -> Vec<Uuid> {
    let Some(element) =
        (unsafe { record.getAttributeDataElement(SERVICE_CLASS_ID_LIST_ATTRIBUTE_ID) })
    else {
        debug!("record has no ServiceClassIDList attribute");
        return Vec::new();
    };
    // IOBluetooth surfaces a single-UUID ServiceClassIDList as a bare UUID element, not a 1-item array.
    if let Some(uuid) = unsafe { element.getUUIDValue() } {
        return uuid_from_sdp_uuid(&uuid).into_iter().collect();
    }

    // getArrayValue() doesn't recognize this element even when it's an array; getValue() + downcast does.
    let sub_elements =
        unsafe { element.getValue() }.and_then(|value| value.downcast::<NSArray<AnyObject>>().ok());
    let Some(sub_elements) = sub_elements else {
        debug!(
            "ServiceClassIDList attribute value is not an array (type descriptor: {:?})",
            unsafe { element.getTypeDescriptor() }
        );
        return Vec::new();
    };
    let sub_elements = downcast_all::<IOBluetoothSDPDataElement>(&sub_elements);
    debug!("ServiceClassIDList has {} entries", sub_elements.len());
    sub_elements
        .into_iter()
        .filter_map(|sub_element| {
            let uuid = unsafe { sub_element.getUUIDValue() };
            if uuid.is_none() {
                debug!("ServiceClassIDList entry is not a UUID");
            }
            uuid.and_then(|uuid| uuid_from_sdp_uuid(&uuid))
        })
        .collect()
}

fn sdp_uuid_from_uuid(uuid: Uuid) -> Option<Retained<IOBluetoothSDPUUID>> {
    let bytes = uuid.into_bytes();
    unsafe { IOBluetoothSDPUUID::uuidWithBytes_length(bytes.as_ptr().cast(), bytes.len() as _) }
}

fn uuid_from_sdp_uuid(sdp_uuid: &IOBluetoothSDPUUID) -> Option<Uuid> {
    let normalized = unsafe { sdp_uuid.getUUIDWithLength(16) }?;
    let bytes: [u8; 16] = normalized.to_vec().try_into().ok()?;
    Some(Uuid::from_bytes(bytes))
}

struct ConnectionState {
    open_complete_sender: std::sync::mpsc::Sender<c_int>,
    data_sender: mpsc::Sender<Vec<u8>>,
    status_sender: watch::Sender<connection::ConnectionStatus>,
    running: Arc<AtomicBool>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = ConnectionState]
    struct RfcommDelegate;

    unsafe impl NSObjectProtocol for RfcommDelegate {}

    impl RfcommDelegate {
        #[unsafe(method(rfcommChannelOpenComplete:status:))]
        fn rfcomm_channel_open_complete(
            &self,
            _channel: Option<&IOBluetoothRFCOMMChannel>,
            status: c_int,
        ) {
            debug!("rfcommChannelOpenComplete status: {status}");
            let _ = self.ivars().open_complete_sender.send(status);
        }

        #[unsafe(method(rfcommChannelData:data:length:))]
        fn rfcomm_channel_data(
            &self,
            _channel: Option<&IOBluetoothRFCOMMChannel>,
            data: *mut c_void,
            length: usize,
        ) {
            let bytes = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), length) }.to_vec();
            trace!("received packet: {bytes:?}");
            // This callback runs on the connection thread, which also services writes and pumps
            // the run loop, so blocking here would freeze the whole connection if the reader
            // ever stalls: drop the packet instead of waiting for buffer space.
            match self.ivars().data_sender.try_send(bytes) {
                Ok(()) => {}
                Err(mpsc::error::TrySendError::Full(_)) => {
                    debug!("read_channel buffer full, dropping packet");
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    debug!("read_channel receiver is closed");
                }
            }
        }

        #[unsafe(method(rfcommChannelClosed:))]
        fn rfcomm_channel_closed(&self, _channel: Option<&IOBluetoothRFCOMMChannel>) {
            debug!("rfcomm channel closed");
            self.ivars().running.store(false, Ordering::Relaxed);
            self.ivars()
                .status_sender
                .send_replace(connection::ConnectionStatus::Disconnected);
            if let Some(run_loop) = CFRunLoop::current() {
                run_loop.stop();
            }
        }
    }
);

impl RfcommDelegate {
    fn new(
        open_complete_sender: std::sync::mpsc::Sender<c_int>,
        data_sender: mpsc::Sender<Vec<u8>>,
        status_sender: watch::Sender<connection::ConnectionStatus>,
        running: Arc<AtomicBool>,
    ) -> Retained<Self> {
        let this = Self::alloc().set_ivars(ConnectionState {
            open_complete_sender,
            data_sender,
            running,
            status_sender,
        });
        unsafe { objc2::msg_send![super(this), init] }
    }
}

struct SendableDevice(Retained<IOBluetoothDevice>);
unsafe impl Send for SendableDevice {}

pub struct MacosRfcommConnection {
    write_requests:
        std::sync::mpsc::Sender<(Vec<u8>, std::sync::mpsc::Sender<connection::Result<()>>)>,
    read_channel: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    connection_status_receiver: watch::Receiver<connection::ConnectionStatus>,
    running: Arc<AtomicBool>,
}

impl MacosRfcommConnection {
    fn open(
        device: Retained<IOBluetoothDevice>,
        channel_id: BluetoothRFCOMMChannelID,
    ) -> connection::Result<Self> {
        let (data_sender, data_receiver) = mpsc::channel(100);
        let (status_sender, status_receiver) =
            watch::channel(connection::ConnectionStatus::Connected);
        let (open_result_sender, open_result_receiver) = std::sync::mpsc::channel();
        let (write_request_sender, write_request_receiver) = std::sync::mpsc::channel();
        let device = SendableDevice(device);
        let running = Arc::new(AtomicBool::new(true));

        thread::spawn({
            let running = running.clone();
            move || {
                run_connection_thread(
                    device,
                    channel_id,
                    data_sender,
                    status_sender,
                    open_result_sender,
                    write_request_receiver,
                    running,
                )
            }
        });

        open_result_receiver
            .recv()
            .map_err(|err| connection::Error::Other {
                source: Box::new(err),
                location: Location::caller(),
            })??;

        Ok(Self {
            write_requests: write_request_sender,
            read_channel: Mutex::new(Some(data_receiver)),
            connection_status_receiver: status_receiver,
            running,
        })
    }
}

// Only the callback counts: isOpen() flips to true a few hundred ms before
// rfcommChannelOpenComplete fires, and writes in that window fail (observed on macOS 15).
fn wait_for_open_complete(
    open_complete_receiver: &std::sync::mpsc::Receiver<c_int>,
) -> connection::Result<()> {
    let started_at = std::time::Instant::now();
    loop {
        match open_complete_receiver.try_recv() {
            Ok(0) => return Ok(()),
            Ok(status) => {
                debug!("RFCOMM channel failed to open with status {status}");
                return Err(connection::Error::DeviceNotFound {
                    source: None,
                    location: Location::caller(),
                });
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err(connection::Error::DeviceNotFound {
                    source: None,
                    location: Location::caller(),
                });
            }
        }
        if started_at.elapsed() >= RFCOMM_OPEN_TIMEOUT {
            debug!("timed out waiting for rfcommChannelOpenComplete");
            return Err(connection::Error::TimedOut {
                action: "open RFCOMM channel",
            });
        }
        pump_run_loop();
    }
}

fn pump_run_loop() {
    unsafe {
        CFRunLoop::run_in_mode(kCFRunLoopDefaultMode, RFCOMM_EVENT_POLL_SECONDS, false);
    }
}

// writeSync:length: must not be called with more bytes than the channel's negotiated MTU (per
// IOBluetoothRFCOMMChannel's own documented contract). MTU can read back as 0 in the brief
// window right after connecting before it's negotiated (see WRITE_NOT_OPEN_RETRIES below);
// fall back to one unsplit write in that case rather than guessing a default.
fn do_write(channel: &IOBluetoothRFCOMMChannel, data: &[u8]) -> connection::Result<()> {
    let mtu = usize::from(unsafe { channel.getMTU() });
    let chunk_size = if mtu == 0 { data.len().max(1) } else { mtu };
    for chunk in data.chunks(chunk_size) {
        write_chunk(channel, chunk)?;
    }
    trace!("wrote packet: {data:?}");
    Ok(())
}

fn write_chunk(channel: &IOBluetoothRFCOMMChannel, chunk: &[u8]) -> connection::Result<()> {
    let mut buffer = chunk.to_vec();
    let mut status;
    let mut attempt = 0;
    loop {
        status =
            unsafe { channel.writeSync_length(buffer.as_mut_ptr().cast(), buffer.len() as u16) };
        attempt += 1;
        if status != IO_RETURN_NOT_OPEN || attempt >= WRITE_NOT_OPEN_RETRIES {
            break;
        }
        debug!("channel not open yet, retrying write ({attempt})");
        // wait_for_open_complete already confirmed the channel opened once before any write is
        // attempted, so a not-open status here is a transient stall on a live connection, not
        // the initial-open race: pump so a real rfcommChannelClosed: during the stall is caught
        // instead of blocking through it.
        pump_run_loop();
    }
    debug!("writeSync status: {status}, channel isOpen: {}", unsafe {
        channel.isOpen()
    });
    if status != 0 {
        return Err(connection::Error::WriteError {
            source: None,
            location: Location::caller(),
        });
    }
    Ok(())
}

fn run_connection_thread(
    device: SendableDevice,
    channel_id: BluetoothRFCOMMChannelID,
    data_sender: mpsc::Sender<Vec<u8>>,
    status_sender: watch::Sender<connection::ConnectionStatus>,
    open_result_sender: std::sync::mpsc::Sender<connection::Result<()>>,
    write_request_receiver: std::sync::mpsc::Receiver<(
        Vec<u8>,
        std::sync::mpsc::Sender<connection::Result<()>>,
    )>,
    running: Arc<AtomicBool>,
) {
    let span = debug_span!("MacosRfcommConnection connection thread");
    let _span_guard = span.enter();

    let (open_complete_sender, open_complete_receiver) = std::sync::mpsc::channel();
    let delegate = RfcommDelegate::new(
        open_complete_sender,
        data_sender,
        status_sender.clone(),
        running.clone(),
    );

    let mut channel: Option<Retained<IOBluetoothRFCOMMChannel>> = None;
    let delegate_object: &AnyObject = &delegate;
    let start_status = unsafe {
        device.0.openRFCOMMChannelAsync_withChannelID_delegate(
            Some(&mut channel),
            channel_id,
            Some(delegate_object),
        )
    };

    let Some(channel) = channel.filter(|_| start_status == 0) else {
        debug!("openRFCOMMChannelAsync failed to start with status {start_status}");
        let _ = open_result_sender.send(Err(connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        }));
        return;
    };

    // openRFCOMMChannelAsync returning kIOReturnSuccess only means the open was started. The
    // channel isn't usable until rfcommChannelOpenComplete fires, and if it never does (see
    // cycle_baseband_connection) writes fail with kIOReturnNotOpen forever, so wait for it here
    // rather than discovering that on the first write. The callback may be delivered on this
    // thread's run loop or the main thread's depending on macOS version, so pump ours while
    // waiting and let the channel carry the result either way.
    if let Err(err) = wait_for_open_complete(&open_complete_receiver) {
        unsafe {
            channel.setDelegate(None);
            channel.closeChannel();
        }
        let _ = open_result_sender.send(Err(err));
        return;
    }

    let _ = open_result_sender.send(Ok(()));

    // Empirically, channel callbacks (rfcommChannelData etc.) are delivered on whichever thread
    // pumps the run loop here, unlike SDP query callbacks which need the real main thread
    // (see perform_sdp_query), so writes and pumping stay on this dedicated thread.
    debug!("channel opened, running write/pump loop");

    // Service any write requests queued while the channel was still opening.
    for (data, response_sender) in write_request_receiver.try_iter() {
        let _ = response_sender.send(do_write(&channel, &data));
    }

    while running.load(Ordering::Relaxed) {
        match write_request_receiver.recv_timeout(WRITE_DRAIN_TIMEOUT) {
            Ok((data, response_sender)) => {
                let _ = response_sender.send(do_write(&channel, &data));
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
        if running.load(Ordering::Relaxed) {
            pump_run_loop();
        }
    }

    debug!("run loop stopped, draining pending write requests");
    // The caller's `write()` is blocked on recv() waiting for a status. If we exit without a
    // response it would hang forever, so answer all still-queued writes with a clean failure
    // before closing the channel.
    for (_, response_sender) in write_request_receiver.try_iter() {
        let _ = response_sender.send(Err(connection::Error::WriteError {
            source: None,
            location: Location::caller(),
        }));
    }

    debug!("cleaning up channel");
    unsafe {
        channel.setDelegate(None);
        channel.closeChannel();
    }
    // rfcommChannelClosed: (the only other place this is sent) can't fire once the delegate
    // above is cleared, so a locally-initiated close would otherwise leave connection_status()
    // stuck reporting Connected forever.
    status_sender.send_replace(connection::ConnectionStatus::Disconnected);
    debug!("connection thread exiting");
}

#[async_trait]
impl RfcommConnection for MacosRfcommConnection {
    async fn write(&self, data: &[u8]) -> connection::Result<()> {
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        self.write_requests
            .send((data.to_owned(), response_sender))
            .map_err(|err| connection::Error::Other {
                source: Box::new(err),
                location: Location::caller(),
            })?;
        tokio::task::spawn_blocking(move || response_receiver.recv())
            .await
            .expect("connection thread panicked while handling a write")
            .map_err(|err| connection::Error::Other {
                source: Box::new(err),
                location: Location::caller(),
            })?
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.read_channel
            .lock()
            .unwrap()
            .take()
            .expect("read_channel may only be called once per MacosRfcommConnection")
    }

    fn connection_status(&self) -> watch::Receiver<connection::ConnectionStatus> {
        self.connection_status_receiver.clone()
    }
}

impl Drop for MacosRfcommConnection {
    fn drop(&mut self) {
        // The write-request sender is dropped with this struct, which unblocks the connection
        // thread (it sees Disconnected, or running flips and it exits), and it cleans up the
        // channel (clears the delegate, closes it) on its own thread.
        self.running.store(false, Ordering::Relaxed);
    }
}
