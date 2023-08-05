// Automatically generated by flapigen
package com.oppzippy.openscq30.lib.bindings;


public enum NoiseCancelingMode {
    Transport(0),
    Outdoor(1),
    Indoor(2),
    Custom(3);

    private final int value;
    NoiseCancelingMode(int value) {
        this.value = value;
    }
    public final int getValue() { return value; }
    /*package*/ static NoiseCancelingMode fromInt(int x) {
        switch (x) {
            case 0: return Transport;
            case 1: return Outdoor;
            case 2: return Indoor;
            case 3: return Custom;
            default: throw new Error("Invalid value for enum NoiseCancelingMode: " + x);
        }
    }
}