package com.longport.quote;

import java.util.Arrays;

/**
 * Market participant (broker) information.
 */
public class ParticipantInfo {
    private int[] brokerIds;
    private String nameCn;
    private String nameEn;
    private String nameHk;

    /**
     * Returns the broker IDs of this participant.
     *
     * @return the broker IDs
     */
    public int[] getBrokerIds() {
        return brokerIds;
    }

    /**
     * Returns the participant name in simplified Chinese.
     *
     * @return the participant name in simplified Chinese
     */
    public String getNameCn() {
        return nameCn;
    }

    /**
     * Returns the participant name in English.
     *
     * @return the participant name in English
     */
    public String getNameEn() {
        return nameEn;
    }

    /**
     * Returns the participant name in traditional Chinese.
     *
     * @return the participant name in traditional Chinese
     */
    public String getNameHk() {
        return nameHk;
    }

    @Override
    public String toString() {
        return "ParticipantInfo [brokerIds=" + Arrays.toString(brokerIds) + ", nameCn=" + nameCn + ", nameEn=" + nameEn
                + ", nameHk=" + nameHk + "]";
    }
}
