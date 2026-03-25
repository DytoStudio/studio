import { Result } from '@resulted/results';

/**
 * The level of a log message, indicating its severity or importance.
 * 0 = Debug: Detailed information for debugging purposes.
 * 1 = Info: General informational messages about application operation.
 * 2 = Warn: Indications of potential issues or important events that are not
 *           errors.
 * 3 = Error: Serious issues that indicate a failure in the application.
 */
export type LogLevel = 0 | 1 | 2 | 3;

/**
 * A function type for logging messages, which can be implemented to integrate
 * with external logging systems or to customize log output.
 * @param level The severity level of the log message.
 * @param module The module or area of the application that is generating the
 *               log message.
 * @param message The main log message to be recorded.
 * @param details Optional additional details or context to include with the log
 *                message, which can be of any type.
 */
export type LoggingFunction = (
    level: LogLevel,
    module: string,
    message: string,
    details: unknown,
) => void;

/**
 * A dependency-injected logger class.
 */
export class Logger {
    /**
     * The global logging function used by all loggers.
     * Can be overridden to integrate with external logging systems.
     */
    private static logFunction: LoggingFunction | null = null;
    /**
     * The global logging level threshold.
     */
    private static logLevel: LogLevel | null = null;

    /**
     * The module / area of the application this logger is associated with.
     */
    private module: string;

    /**
     * Private constructor to enforce the use of the static `new` method for
     * instantiation.
     * @param module The module or area of the application this logger is
     *               associated with.
     */
    private constructor(module: string) {
        this.module = module;
    }

    /**
     * Creates a new logger instance for the specified module.
     * @param module The module or area of the application this logger is
     *               associated with.
     * @returns A new logger instance with the specified prefix.
     */
    public static new(module: string): Logger {
        return new Logger(module);
    }

    /**
     * Sets the global logging function to be used by all loggers.
     * @param logFunction The logging function to set.
     * @returns A result indicating success or failure if the log function has
     *          already been set.
     */
    public static setLogFunction(
        logFunction: LoggingFunction,
    ): Result<void, 'log-function-already-set'> {
        if (Logger.logFunction) return Result.err('log-function-already-set');
        Logger.logFunction = logFunction;
        return Result.ok(undefined);
    }

    /**
     * Sets the global logging level threshold. Messages below this level will
     * not be logged.
     * @param level The logging level threshold to set.
     * @returns A result indicating success or failure if the log level has
     *          already been set.
     */
    public static setLogLevel(
        level: LogLevel,
    ): Result<void, 'log-level-already-set'> {
        if (Logger.logLevel) return Result.err('log-level-already-set');
        Logger.logLevel = level;
        return Result.ok(undefined);
    }

    /**
     * An internal method to log a message at the specified level with optional
     * details.
     */
    private log(level: LogLevel, message: string, details: unknown): void {
        if (
            !Logger.logFunction ||
            Logger.logLevel === null ||
            level < Logger.logLevel
        )
            return;

        Logger.logFunction(level, this.module, message, details);
    }

    /**
     * Log a debug message.
     * @param message The message to log.
     * @param details Optional additional details to include with the log.
     */
    public debug(message: string, details: unknown = null): void {
        this.log(0, message, details);
    }

    /**
     * Log an informational message.
     * @param message The message to log.
     * @param details Optional additional details to include with the log.
     */
    public info(message: string, details: unknown = null): void {
        this.log(1, message, details);
    }

    /**
     * Log a warning message.
     * @param message The message to log.
     * @param details Optional additional details to include with the log.
     */
    public warn(message: string, details: unknown = null): void {
        this.log(2, message, details);
    }

    /**
     * Log an error message.
     * @param message The message to log.
     * @param details Optional additional details to include with the log.
     */
    public error(message: string, details: unknown = null): void {
        this.log(3, message, details);
    }

    /**
     * Log a error message then throw an error.
     * @param message The message to log and include in the thrown error.
     * @param details Optional additional details to include with the log.
     */
    public errorAndThrow(message: string, details: unknown = null): never {
        this.error(message, details);
        // safety: this method is intended to be used in situations where an
        //         error is required
        throw new Error(message);
    }
}
