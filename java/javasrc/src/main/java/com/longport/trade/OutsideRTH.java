package com.longport.trade;

/**
 * Whether the order is allowed to be traded outside regular trading hours
 */
public enum OutsideRTH {
    /** Unknown */
    Unknown,
    /** Regular trading hours only */
    RTHOnly,
    /** Any time (including pre/post market) */
    AnyTime,
    /** Overnight session */
    Overnight,
}
