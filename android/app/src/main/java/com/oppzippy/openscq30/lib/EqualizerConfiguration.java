// Automatically generated by flapigen
package com.oppzippy.openscq30.lib;
import androidx.annotation.NonNull;

public final class EqualizerConfiguration {

    public EqualizerConfiguration(@NonNull PresetEqualizerProfile preset_profile) {
        int a0 = preset_profile.getValue();
        mNativeObj = init(a0);
        JNIReachabilityFence.reachabilityFence1(preset_profile);
    }
    private static native long init(int preset_profile);

    public EqualizerConfiguration(@NonNull EqualizerBandOffsets band_offsets) {
        long a0 = band_offsets.mNativeObj;
        mNativeObj = init(a0);
        JNIReachabilityFence.reachabilityFence1(band_offsets);
    }
    private static native long init(long band_offsets);

    public final int profileId() {
        int ret = do_profileId(mNativeObj);

        return ret;
    }
    private static native int do_profileId(long self);

    public final @NonNull java.util.Optional<PresetEqualizerProfile> presetProfile() {
        int ret = do_presetProfile(mNativeObj);
        java.util.Optional<PresetEqualizerProfile> convRet;
        if (ret != -1) {
            convRet = java.util.Optional.of(PresetEqualizerProfile.fromInt(ret));
        } else {
            convRet = java.util.Optional.empty();
        }

        return convRet;
    }
    private static native int do_presetProfile(long self);

    public final @NonNull EqualizerBandOffsets bandOffsets() {
        long ret = do_bandOffsets(mNativeObj);
        EqualizerBandOffsets convRet = new EqualizerBandOffsets(InternalPointerMarker.RAW_PTR, ret);

        return convRet;
    }
    private static native long do_bandOffsets(long self);

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ EqualizerConfiguration(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}