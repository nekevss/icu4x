// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { Date } from "./Date"
import type { DateTimeAlignment } from "./DateTimeAlignment"
import type { DateTimeFormatterLoadError } from "./DateTimeFormatterLoadError"
import type { DateTimeLength } from "./DateTimeLength"
import type { DateTimeMismatchedCalendarError } from "./DateTimeMismatchedCalendarError"
import type { IsoDate } from "./IsoDate"
import type { Locale } from "./Locale"
import type { Time } from "./Time"
import type { TimePrecision } from "./TimePrecision"
import type { YearStyle } from "./YearStyle"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** 
 * See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/type.DateTimeFormatter.html) for more information.
 */


export class DateTimeFormatter {
    
    get ffiValue(): pointer;

    /** 
     * See the [Rust documentation for `DT`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html#method.with_alignment)
     */
    static createDt(locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `DT`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html) for more information.
     *
     * See the [Rust documentation for `DT`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DT.html#method.with_alignment)
     */
    static createDtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `MDT`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html#method.with_alignment)
     */
    static createMdt(locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `MDT`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDT.html#method.with_alignment)
     */
    static createMdtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `YMDT`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_alignment), [4](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_year_style)
     */
    static createYmdt(locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null, yearStyle: YearStyle | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `YMDT`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_alignment), [4](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html#method.with_year_style)
     */
    static createYmdtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null, yearStyle: YearStyle | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `DET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html#method.with_alignment)
     */
    static createDet(locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `DET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.DET.html#method.with_alignment)
     */
    static createDetWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `MDET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html#method.with_alignment)
     */
    static createMdet(locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `MDET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.MDET.html#method.with_alignment)
     */
    static createMdetWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `YMDET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_alignment), [4](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_year_style)
     */
    static createYmdet(locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null, yearStyle: YearStyle | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `YMDET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_alignment), [4](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDET.html#method.with_year_style)
     */
    static createYmdetWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null, yearStyle: YearStyle | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `ET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html#method.with_alignment)
     */
    static createEt(locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `ET`](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html) for more information.
     *
     * Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html#method.with_length), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html#method.with_time_precision), [3](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.ET.html#method.with_alignment)
     */
    static createEtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength | null, timePrecision: TimePrecision | null, alignment: DateTimeAlignment | null): DateTimeFormatter;

    /** 
     * See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
     */
    formatIso(date: IsoDate, time: Time): string;

    /** 
     * See the [Rust documentation for `format_same_calendar`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format_same_calendar) for more information.
     */
    formatSameCalendar(date: Date, time: Time): string;
}