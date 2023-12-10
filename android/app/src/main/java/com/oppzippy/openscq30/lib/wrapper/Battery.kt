package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.battery
import com.oppzippy.openscq30.lib.protobuf.dualBattery
import com.oppzippy.openscq30.lib.protobuf.singleBattery

sealed class Battery {
    class Single(val singleBattery: SingleBattery) : Battery()
    class Dual(val dualBattery: DualBattery) : Battery()

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.Battery {
        return when (this) {
            is Single -> battery { singleBattery = this@Battery.singleBattery.toProtobuf() }
            is Dual -> battery { dualBattery = this@Battery.dualBattery.toProtobuf() }
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.Battery.toKotlin(): Battery {
    return when (this.batteryCase) {
        com.oppzippy.openscq30.lib.protobuf.Battery.BatteryCase.SINGLE_BATTERY -> Battery.Single(
            singleBattery.toKotlin(),
        )

        com.oppzippy.openscq30.lib.protobuf.Battery.BatteryCase.DUAL_BATTERY -> Battery.Dual(
            dualBattery.toKotlin(),
        )

        com.oppzippy.openscq30.lib.protobuf.Battery.BatteryCase.BATTERY_NOT_SET -> TODO()
        null -> TODO()
    }
}

data class DualBattery(
    val left: SingleBattery,
    val right: SingleBattery,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.DualBattery {
        return dualBattery {
            left = this@DualBattery.left.toProtobuf()
            right = this@DualBattery.right.toProtobuf()
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.DualBattery.toKotlin(): DualBattery {
    return DualBattery(left = left.toKotlin(), right = right.toKotlin())
}

data class SingleBattery(
    val isCharging: Boolean,
    val level: UByte,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.SingleBattery {
        return singleBattery {
            isCharging = this@SingleBattery.isCharging
            level = this@SingleBattery.level.toInt()
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.SingleBattery.toKotlin(): SingleBattery {
    return SingleBattery(isCharging = isCharging, level = level.toUByte())
}
