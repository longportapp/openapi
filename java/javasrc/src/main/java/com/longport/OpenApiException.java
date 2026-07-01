package com.longport;

/**
 * Exception thrown by the Longport OpenAPI SDK.
 * <p>
 * Every SDK operation that can fail will throw this exception. It carries three
 * pieces of diagnostic information:
 * <ul>
 *   <li>{@link #getKind()} – broad category of the error (HTTP, OpenAPI, OAuth, Other)</li>
 *   <li>{@link #getCode()} – numeric error code returned by the server, or {@code null} for
 *       client-side errors</li>
 *   <li>{@link #getMessage()} – human-readable error description</li>
 * </ul>
 */
public class OpenApiException extends Exception {
    private ErrorKind kind;
    private Long code;
    private String message;

    /**
     * Constructs an {@code OpenApiException}.
     *
     * @param kind    error kind
     * @param code    numeric error code (may be {@code null})
     * @param message human-readable error description
     */
    public OpenApiException(ErrorKind kind, Long code, String message) {
        this.kind = kind;
        this.code = code;
        this.message = message;
    }

    /**
     * Returns the error kind.
     *
     * @return error kind
     */
    public ErrorKind getKind() {
        return kind;
    }

    /**
     * Returns the numeric error code returned by the server.
     *
     * @return error code, or {@code null} for client-side errors
     */
    public Long getCode() {
        return code;
    }

    /**
     * Returns the human-readable error description.
     *
     * @return error message
     */
    public String getMessage() {
        return message;
    }

    @Override
    public String toString() {
        return "OpenApiException [kind=" + kind + ", code=" + code + ", message=" + message + "]";
    }
}
