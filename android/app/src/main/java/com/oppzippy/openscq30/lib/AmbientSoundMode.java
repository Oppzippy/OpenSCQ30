// Automatically generated by flapigen
package com.oppzippy.openscq30.lib;


public enum AmbientSoundMode {
    NoiseCanceling(0),
    Transparency(1),
    Normal(2);

    private final int value;
    AmbientSoundMode(int value) {
        this.value = value;
    }
    public final int getValue() { return value; }
    /*package*/ static AmbientSoundMode fromInt(int x) {
        switch (x) {
            case 0: return NoiseCanceling;
            case 1: return Transparency;
            case 2: return Normal;
            default: throw new Error("Invalid value for enum AmbientSoundMode: " + x);
        }
    }
}
