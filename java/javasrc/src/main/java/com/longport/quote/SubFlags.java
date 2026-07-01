package com.longport.quote;

/**
 * Subscription flags for {@link QuoteContext#subscribe}.
 * <p>
 * Combine multiple flags with bitwise OR, e.g.
 * {@code SubFlags.Quote | SubFlags.Depth}.
 */
public class SubFlags {
    /** Quote subscription */
    public static int Quote = 0x1;
    /** Depth subscription */
    public static int Depth = 0x2;
    /** Broker queue subscription */
    public static int Brokers = 0x4;
    /** Trade subscription */
    public static int Trade = 0x8;
}
