// Automatically generated by flapigen
package com.oppzippy.openscq30.lib.bindings;
import androidx.annotation.NonNull;

public final class SoundModes {

    public SoundModes(@NonNull AmbientSoundMode ambient_sound_mode, @NonNull NoiseCancelingMode noise_canceling_mode, @NonNull TransparencyMode transparency_mode, @NonNull CustomNoiseCanceling custom_noise_canceling) {
        int a0 = ambient_sound_mode.getValue();        int a1 = noise_canceling_mode.getValue();        int a2 = transparency_mode.getValue();
        long a3 = custom_noise_canceling.mNativeObj;
        custom_noise_canceling.mNativeObj = 0;

        mNativeObj = init(a0, a1, a2, a3);
        JNIReachabilityFence.reachabilityFence4(ambient_sound_mode, noise_canceling_mode, transparency_mode, custom_noise_canceling);
    }
    private static native long init(int ambient_sound_mode, int noise_canceling_mode, int transparency_mode, long custom_noise_canceling);

    public final AmbientSoundMode ambientSoundMode() {
        int ret = do_ambientSoundMode(mNativeObj);
        AmbientSoundMode convRet = AmbientSoundMode.fromInt(ret);

        return convRet;
    }
    private static native int do_ambientSoundMode(long self);

    public final NoiseCancelingMode noiseCancelingMode() {
        int ret = do_noiseCancelingMode(mNativeObj);
        NoiseCancelingMode convRet = NoiseCancelingMode.fromInt(ret);

        return convRet;
    }
    private static native int do_noiseCancelingMode(long self);

    public final TransparencyMode transparencyMode() {
        int ret = do_transparencyMode(mNativeObj);
        TransparencyMode convRet = TransparencyMode.fromInt(ret);

        return convRet;
    }
    private static native int do_transparencyMode(long self);

    public final @NonNull CustomNoiseCanceling customNoiseCanceling() {
        long ret = do_customNoiseCanceling(mNativeObj);
        CustomNoiseCanceling convRet = new CustomNoiseCanceling(InternalPointerMarker.RAW_PTR, ret);

        return convRet;
    }
    private static native long do_customNoiseCanceling(long self);

    public final boolean innerEquals(@NonNull SoundModes other) {
        long a0 = other.mNativeObj;
        boolean ret = do_innerEquals(mNativeObj, a0);

        JNIReachabilityFence.reachabilityFence1(other);

        return ret;
    }
    private static native boolean do_innerEquals(long self, long other);

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
    /*package*/ SoundModes(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}