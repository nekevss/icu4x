// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! All available field sets for datetime formatting.
//!
//! Each field set is a struct containing options specified to that field set.
//! The fields can either be set directly or via helper functions.
//!
//! This module contains _static_ field sets, which deliver the smallest binary size.
//! If the field set is not known until runtime, use a _dynamic_ field set: [`enums`]
//!
//! # What is a Field Set?
//!
//! A field set determines what datetime fields should be printed in the localized output.
//!
//! Examples of field sets include:
//!
//! 1. Year, month, and day ([`YMD`])
//! 2. Weekday and time ([`ET`])
//!
//! Field sets fit into four categories:
//!
//! 1. Date: fields that specify a particular day in time.
//! 2. Calendar period: fields that specify a span of time greater than a day.
//! 3. Time: fields that specify a time within a day.
//! 4. Zone: fields that specify a time zone or offset from UTC.
//!
//! Certain combinations of field sets are allowed, too. See [`Combo`].
//!
//! # Examples
//!
//! There are two ways to configure the same field set:
//!
//! ```
//! use icu::datetime::fieldsets::YMDT;
//! use icu::datetime::options::{Alignment, TimePrecision, YearStyle};
//!
//! let field_set_1 = YMDT::long()
//!     .with_year_style(YearStyle::Full)
//!     .with_alignment(Alignment::Column)
//!     .with_time_precision(TimePrecision::Minute);
//!
//! let mut field_set_2 = YMDT::long();
//! field_set_2.year_style = Some(YearStyle::Full);
//! field_set_2.alignment = Some(Alignment::Column);
//! field_set_2.time_precision = Some(TimePrecision::Minute);
//!
//! assert_eq!(field_set_1, field_set_2);
//! ```

#[path = "builder.rs"]
pub mod builder;
#[path = "dynamic.rs"]
pub mod enums;

pub use crate::combo::Combo;

use crate::{
    options::*,
    provider::{neo::*, time_zones::tz, *},
    raw::neo::RawOptions,
    scaffold::*,
};
use enums::*;
use icu_calendar::types::{DayOfMonth, MonthInfo, Weekday, YearInfo};
use icu_provider::marker::NeverMarker;
use icu_time::{
    zone::{UtcOffset, ZoneNameTimestamp},
    Hour, Minute, Nanosecond, Second, TimeZone,
};

#[cfg(doc)]
use icu_time::TimeZoneInfo;

/// Maps the token `yes` to the given ident
macro_rules! yes_to {
    ($any:expr, $nonempty:expr) => {
        $any
    };
}

macro_rules! yes_or {
    ($fallback:expr, $actual:expr) => {
        $actual
    };
    ($fallback:expr,) => {
        $fallback
    };
}

macro_rules! ternary {
    ($present:expr, $missing:expr, yes) => {
        $present
    };
    ($present:expr, $missing:expr, $any:literal) => {
        $present
    };
    ($present:expr, $missing:expr,) => {
        $missing
    };
}

/// Generates the options argument passed into the docs test constructor
macro_rules! length_option_helper {
    ($type:ty, $length:ident) => {
        concat!(stringify!($type), "::", stringify!($length), "()")
    };
}

macro_rules! impl_composite {
    ($type:ident, $variant:ident, $enum:ident) => {
        impl $type {
            #[inline]
            pub(crate) fn to_enum(self) -> $enum {
                $enum::$type(self)
            }
        }
        impl GetField<CompositeFieldSet> for $type {
            #[inline]
            fn get_field(&self) -> CompositeFieldSet {
                CompositeFieldSet::$variant(self.to_enum())
            }
        }
    };
}

macro_rules! impl_marker_length_constructors {
    (
        $type:ident,
        $(alignment: $alignment_yes:ident,)?
        $(year_style: $yearstyle_yes:ident,)?
        $(time_precision: $timeprecision_yes:ident,)?
    ) => {
        impl $type {
            #[doc = concat!("Creates a ", stringify!($type), " skeleton with the given formatting length.")]
            pub const fn for_length(length: Length) -> Self {
                Self {
                    length,
                    $(
                        alignment: yes_to!(None, $alignment_yes),
                    )?
                    $(
                        year_style: yes_to!(None, $yearstyle_yes),
                    )?
                    $(
                        time_precision: yes_to!(None, $timeprecision_yes),
                    )?
                }
            }
            #[doc = concat!("Creates a ", stringify!($type), " skeleton with a long length.")]
            pub const fn long() -> Self {
                Self::for_length(Length::Long)
            }
            #[doc = concat!("Creates a ", stringify!($type), " skeleton with a medium length.")]
            pub const fn medium() -> Self {
                Self::for_length(Length::Medium)
            }
            #[doc = concat!("Creates a ", stringify!($type), " skeleton with a short length.")]
            pub const fn short() -> Self {
                Self::for_length(Length::Short)
            }
        }
    };
}

macro_rules! impl_time_precision_constructors {
    (
        $time_type:ident,
    ) => {
        impl $time_type {
            #[doc = concat!("Creates a ", stringify!($type), " that formats hours and minutes with the default length.")]
            pub fn hm() -> Self {
                Self::for_length(Default::default()).with_time_precision(TimePrecision::Minute)
            }
            #[doc = concat!("Creates a ", stringify!($type), " that formats hours, minutes, and seconds with the default length.")]
            pub fn hms() -> Self {
                Self::for_length(Default::default()).with_time_precision(TimePrecision::Second)
            }
            #[doc = concat!("Creates a ", stringify!($type), " that formats hours, minutes, seconds, and subseconds with the default length.")]
            pub fn hmss(subsecond_digits: SubsecondDigits) -> Self {
                Self::for_length(Default::default()).with_time_precision(TimePrecision::Subsecond(subsecond_digits))
            }
        }
    };
}

macro_rules! impl_marker_with_options {
    (
        $(#[$attr:meta])*
        $type:ident,
        $(sample_length: $sample_length:ident,)?
        $(date_fields: $date_fields:expr,)?
        $(alignment: $alignment_yes:ident,)?
        $(year_style: $yearstyle_yes:ident,)?
        $(time_precision: $timeprecision_yes:ident,)?
        $(length_override: $length_override:ident,)?
    ) => {
        $(#[$attr])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[non_exhaustive]
        pub struct $type {
            $(
                /// The desired length of the formatted string.
                ///
                /// See: [`Length`]
                pub length: datetime_marker_helper!(@option/length, $sample_length),
            )?
            $(
                /// Whether fields should be aligned for a column-like layout.
                ///
                /// See: [`Alignment`]
                pub alignment: datetime_marker_helper!(@option/alignment, $alignment_yes),
            )?
            $(
                /// When to display the era field in the formatted string.
                ///
                /// See: [`YearStyle`]
                pub year_style: datetime_marker_helper!(@option/yearstyle, $yearstyle_yes),
            )?
            $(
                /// How precisely to display the time of day
                ///
                /// See: [`TimePrecision`]
                pub time_precision: datetime_marker_helper!(@option/timeprecision, $timeprecision_yes),
            )?
        }
        impl $type {
            pub(crate) fn to_raw_options(self) -> RawOptions {
                RawOptions {
                    length: yes_or!(Some(self.length), $(Some(Length::$length_override))?),
                    date_fields: yes_or!(None, $($date_fields)?),
                    alignment: ternary!(self.alignment, None, $($alignment_yes)?),
                    year_style: ternary!(self.year_style, None, $($yearstyle_yes)?),
                    time_precision: ternary!(self.time_precision, None, $($timeprecision_yes)?),
                }
            }
            /// Builds this field set, removing the needed options from the builder.
            pub(crate) fn take_from_builder(
                options: &mut builder::FieldSetBuilder
            ) -> Self {
                Self {
                    $(length: yes_to!(options.length.take().unwrap_or_default(), $sample_length),)?
                    $(alignment: yes_to!(options.alignment.take(), $alignment_yes),)?
                    $(year_style: yes_to!(options.year_style.take(), $yearstyle_yes),)?
                    $(time_precision: yes_to!(options.time_precision.take(), $timeprecision_yes),)?
                }
            }
        }
        $(
            impl $type {
                /// Sets the length option.
                pub const fn with_length(mut self, length: Length) -> Self {
                    self.length = yes_to!(length, $sample_length);
                    self
                }
            }
        )?
        $(
            impl $type {
                /// Sets the alignment option.
                pub const fn with_alignment(mut self, alignment: Alignment) -> Self {
                    self.alignment = Some(yes_to!(alignment, $alignment_yes));
                    self
                }
            }
        )?
        $(
            impl $type {
                /// Sets the year style option.
                pub const fn with_year_style(mut self, year_style: YearStyle) -> Self {
                    self.year_style = Some(yes_to!(year_style, $yearstyle_yes));
                    self
                }
            }
        )?
        $(
            impl $type {
                /// Sets the time precision option.
                pub const fn with_time_precision(mut self, time_precision: TimePrecision) -> Self {
                    self.time_precision = Some(yes_to!(time_precision, $timeprecision_yes));
                    self
                }
            }
        )?
    };
}

macro_rules! impl_date_to_time_helpers {
    (
        $type:ident,
        $type_time:ident,
        $(alignment = $alignment_yes:ident,)?
        $(year_style = $yearstyle_yes:ident,)?
    ) => {
        impl $type {
            /// Associates this field set with a time precision.
            pub fn with_time(self, time_precision: TimePrecision) -> $type_time {
                $type_time {
                    length: self.length,
                    time_precision: Some(time_precision),
                    alignment: ternary!(self.alignment, Default::default(), $($alignment_yes)?),
                    $(year_style: yes_to!(self.year_style, $yearstyle_yes),)?
                }
            }
            /// Shorthand to associate this field set with [`TimePrecision::Minute`].
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type), ";")]
            /// use icu::datetime::options::TimePrecision;
            ///
            #[doc = concat!("let fs1 = ", stringify!($type), "::medium().with_time(TimePrecision::Minute);")]
            #[doc = concat!("let fs2 = ", stringify!($type), "::medium().with_time_hm();")]
            /// assert_eq!(fs1, fs2);
            /// ```
            pub fn with_time_hm(self) -> $type_time {
                $type_time {
                    length: self.length,
                    time_precision: Some(TimePrecision::Minute),
                    alignment: ternary!(self.alignment, Default::default(), $($alignment_yes)?),
                    $(year_style: yes_to!(self.year_style, $yearstyle_yes),)?
                }
            }
            /// Shorthand to associate this field set with [`TimePrecision::Second`].
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type), ";")]
            /// use icu::datetime::options::TimePrecision;
            ///
            #[doc = concat!("let fs1 = ", stringify!($type), "::medium().with_time(TimePrecision::Second);")]
            #[doc = concat!("let fs2 = ", stringify!($type), "::medium().with_time_hms();")]
            /// assert_eq!(fs1, fs2);
            /// ```
            pub fn with_time_hms(self) -> $type_time {
                $type_time {
                    length: self.length,
                    time_precision: Some(TimePrecision::Second),
                    alignment: ternary!(self.alignment, Default::default(), $($alignment_yes)?),
                    $(year_style: yes_to!(self.year_style, $yearstyle_yes),)?
                }
            }
            /// Shorthand to associate this field set with [`TimePrecision::Subsecond`].
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type), ";")]
            /// use icu::datetime::options::TimePrecision;
            /// use icu::datetime::options::SubsecondDigits::S2;
            ///
            #[doc = concat!("let fs1 = ", stringify!($type), "::medium().with_time(TimePrecision::Subsecond(S2));")]
            #[doc = concat!("let fs2 = ", stringify!($type), "::medium().with_time_hmss(S2);")]
            /// assert_eq!(fs1, fs2);
            /// ```
            pub fn with_time_hmss(self, subsecond_digits: SubsecondDigits) -> $type_time {
                $type_time {
                    length: self.length,
                    time_precision: Some(TimePrecision::Subsecond(subsecond_digits)),
                    alignment: ternary!(self.alignment, Default::default(), $($alignment_yes)?),
                    $(year_style: yes_to!(self.year_style, $yearstyle_yes),)?
                }
            }
        }
    };
}

macro_rules! impl_combo_get_field {
    ($type:ident, $composite:ident, $enum:ident, $variant:path) => {
        impl GetField<CompositeFieldSet> for Combo<$type, $variant> {
            #[inline]
            fn get_field(&self) -> CompositeFieldSet {
                CompositeFieldSet::$composite(Combo::new(self.dt().to_enum(), self.z().to_enum()))
            }
        }
        impl Combo<$type, $variant> {
            /// Convert this specific [`Combo`] into a more general [`Combo`].
            /// Useful when adding to the field of a [`CompositeFieldSet`].
            ///
            /// [`CompositeFieldSet`]: enums::CompositeFieldSet
            pub fn into_enums(self) -> Combo<$enum, ZoneFieldSet> {
                Combo::new(self.dt().to_enum(), self.z().to_enum())
            }
        }
    };
}

macro_rules! impl_zone_combo_helpers {
    (
        $type:ident,
        $composite:ident,
        $enum:ident
    ) => {
        impl $type {
            #[inline]
            /// Associates this field set with a time zone field set.
            pub fn with_zone<Z: ZoneMarkers>(self, zone: Z) -> Combo<Self, Z> {
                Combo::new(self, zone)
            }
        }
        impl_combo_get_field!($type, $composite, $enum, zone::SpecificLong);
        impl_combo_get_field!($type, $composite, $enum, zone::SpecificShort);
        impl_combo_get_field!($type, $composite, $enum, zone::LocalizedOffsetLong);
        impl_combo_get_field!($type, $composite, $enum, zone::LocalizedOffsetShort);
        impl_combo_get_field!($type, $composite, $enum, zone::GenericLong);
        impl_combo_get_field!($type, $composite, $enum, zone::GenericShort);
        impl_combo_get_field!($type, $composite, $enum, zone::Location);
        impl_combo_get_field!($type, $composite, $enum, zone::ExemplarCity);
        impl_combo_get_field!($type, $composite, $enum, ZoneFieldSet);
    };
}

/// Internal helper macro used by [`impl_date_marker`] and [`impl_calendar_period_marker`]
macro_rules! impl_date_or_calendar_period_marker {
    (
        $(#[$attr:meta])*
        // The name of the type being created.
        $type:ident,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether years can occur.
        $(years = $years_yes:ident,)?
        // Whether months can occur.
        $(months = $months_yes:ident,)?
        // Whether weekdays can occur.
        $(weekdays = $weekdays_yes:ident,)?
        // Whether the input should contain years.
        $(input_year = $year_yes:ident,)?
        // Whether the input should contain months.
        $(input_month = $month_yes:ident,)?
        // Whether the input should contain the day of the month.
        $(input_day_of_month = $day_of_month_yes:ident,)?
        // Whether the input should contain the day of the week.
        $(input_day_of_week = $day_of_week_yes:ident,)?
        // Whether the input should contain the day of the year.
        $(input_day_of_year = $day_of_year_yes:ident,)?
        // Whether the input should declare its calendar kind.
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        // Whether the alignment option should be available.
        // According to UTS 35, it should be available with years, months, and days.
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
            ///
            /// This is a field set marker. For more information, see [`fieldsets`](crate::fieldsets).
            ///
            /// # Examples
            ///
            /// In [`DateTimeFormatter`](crate::neo::DateTimeFormatter):
            ///
            /// ```
            /// use icu::datetime::input::Date;
            /// use icu::datetime::DateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type), ";")]
            /// use icu::locale::locale;
            /// use writeable::assert_writeable_eq;
            #[doc = concat!("let fmt = DateTimeFormatter::<", stringify!($type), ">::try_new(")]
            ///     locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Date::try_new_iso(2024, 5, 17).unwrap();
            ///
            /// assert_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            ///
            /// In [`FixedCalendarDateTimeFormatter`](crate::neo::FixedCalendarDateTimeFormatter):
            ///
            /// ```
            /// use icu::datetime::input::Date;
            /// use icu::calendar::Gregorian;
            /// use icu::datetime::FixedCalendarDateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type), ";")]
            /// use icu::locale::locale;
            /// use writeable::assert_writeable_eq;
            ///
            #[doc = concat!("let fmt = FixedCalendarDateTimeFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
            ///     locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Date::try_new_gregorian(2024, 5, 17).unwrap();
            ///
            /// assert_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            $(#[$attr])*
            $type,
            sample_length: $sample_length,
            date_fields: Some(builder::DateFields::$type),
            $(alignment: $option_alignment_yes,)?
            $(year_style: $year_yes,)?
        );
        impl_marker_length_constructors!(
            $type,
            $(alignment: $option_alignment_yes,)?
            $(year_style: $year_yes,)?
        );
        impl UnstableSealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year, $($years_yes)?);
            type MonthNames = datetime_marker_helper!(@names/month, $($months_yes)?);
            type WeekdayNames = datetime_marker_helper!(@names/weekday, $($weekdays_yes)?);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
            type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
            type ZoneLocationsRoot = datetime_marker_helper!(@names/zone/locations_root,);
            type ZoneExemplars = datetime_marker_helper!(@names/zone/exemplar,);
            type ZoneExemplarsRoot = datetime_marker_helper!(@names/zone/exemplar_root,);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
            type ZoneStandardLong = datetime_marker_helper!(@names/zone/standard_long,);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
            type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
        }
        impl DateInputMarkers for $type {
            type YearInput = datetime_marker_helper!(@input/year, $($year_yes)?);
            type MonthInput = datetime_marker_helper!(@input/month, $($month_yes)?);
            type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, $($day_of_month_yes)?);
            type DayOfYearInput = datetime_marker_helper!(@input/day_of_year, $($day_of_year_yes)?);
            type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, $($day_of_week_yes)?);
        }
        impl<C: CldrCalendar> TypedDateDataMarkers<C> for $type {
            type DateSkeletonPatternsV1 = datetime_marker_helper!(@dates/typed, yes);
            type YearNamesV1 = datetime_marker_helper!(@years/typed, $($years_yes)?);
            type MonthNamesV1 = datetime_marker_helper!(@months/typed, $($months_yes)?);
            type WeekdayNamesV1 = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl DateDataMarkers for $type {
            type Skel = datetime_marker_helper!(@calmarkers, yes);
            type Year = datetime_marker_helper!(@calmarkers, $($years_yes)?);
            type Month = datetime_marker_helper!(@calmarkers, $($months_yes)?);
            type WeekdayNamesV1 = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = Self;
            type T = ();
            type Z = ();
            type GluePatternV1 = datetime_marker_helper!(@glue,);
        }
    };
}

/// Implements a field set of date fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// See [`impl_date_marker`].
macro_rules! impl_date_marker {
    (
        $(#[$attr:meta])*
        $type:ident,
        $type_time:ident,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        sample_time = $sample_time:literal,
        $(years = $years_yes:ident,)?
        $(months = $months_yes:ident,)?
        $(dates = $dates_yes:ident,)?
        $(weekdays = $weekdays_yes:ident,)?
        $(input_year = $year_yes:ident,)?
        $(input_month = $month_yes:ident,)?
        $(input_day_of_month = $day_of_month_yes:ident,)?
        $(input_day_of_week = $day_of_week_yes:ident,)?
        $(input_day_of_year = $day_of_year_yes:ident,)?
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_date_or_calendar_period_marker!(
            $(#[$attr])*
            $type,
            description = $description,
            sample_length = $sample_length,
            sample = $sample,
            $(years = $years_yes,)?
            $(months = $months_yes,)?
            $(dates = $dates_yes,)?
            $(weekdays = $weekdays_yes,)?
            $(input_year = $year_yes,)?
            $(input_month = $month_yes,)?
            $(input_day_of_month = $day_of_month_yes,)?
            $(input_day_of_week = $day_of_week_yes,)?
            $(input_day_of_year = $day_of_year_yes,)?
            $(input_any_calendar_kind = $any_calendar_kind_yes,)?
            $(option_alignment = $option_alignment_yes,)?
        );
        impl_zone_combo_helpers!($type, DateZone, DateFieldSet);
        impl_composite!($type, Date, DateFieldSet);
        impl_date_to_time_helpers!($type, $type_time, $(alignment = $option_alignment_yes,)? $(year_style = $year_yes,)?);
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample_time, "**” ⇒ ", $description, " with time")]
            ///
            /// # Examples
            ///
            /// In [`DateTimeFormatter`](crate::neo::DateTimeFormatter):
            ///
            /// ```
            /// use icu::datetime::input::Date;
            /// use icu::datetime::DateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type_time), ";")]
            /// use icu::locale::locale;
            /// use icu::datetime::input::{DateTime, Time};
            /// use writeable::assert_writeable_eq;
            ///
            #[doc = concat!("let fmt = DateTimeFormatter::try_new(")]
            ///     locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type_time, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = DateTime { date: Date::try_new_iso(2024, 5, 17).unwrap(), time: Time::try_new(15, 47, 50, 0).unwrap() };
            ///
            /// assert_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample_time, "\"")]
            /// );
            /// ```
            ///
            /// In [`FixedCalendarDateTimeFormatter`](crate::neo::FixedCalendarDateTimeFormatter):
            ///
            /// ```
            /// use icu::datetime::input::Date;
            /// use icu::calendar::Gregorian;
            /// use icu::datetime::FixedCalendarDateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type_time), ";")]
            /// use icu::locale::locale;
            /// use icu::datetime::input::{DateTime, Time};
            /// use writeable::assert_writeable_eq;
            ///
            #[doc = concat!("let fmt = FixedCalendarDateTimeFormatter::try_new(")]
            ///     locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type_time, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = DateTime { date: Date::try_new_gregorian(2024, 5, 17).unwrap(), time: Time::try_new(15, 47, 50, 0).unwrap() };
            ///
            /// assert_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample_time, "\"")]
            /// );
            /// ```
            $(#[$attr])*
            $type_time,
            sample_length: $sample_length,
            date_fields: Some(builder::DateFields::$type),
            alignment: yes,
            $(year_style: $year_yes,)?
            time_precision: yes,
        );
        impl_marker_length_constructors!(
            $type_time,
            alignment: yes,
            $(year_style: $year_yes,)?
            time_precision: yes,
        );
        impl_zone_combo_helpers!($type_time, DateTimeZone, DateAndTimeFieldSet);
        impl UnstableSealed for $type_time {}
        impl DateTimeNamesMarker for $type_time {
            type YearNames = datetime_marker_helper!(@names/year, $($years_yes)?);
            type MonthNames = datetime_marker_helper!(@names/month, $($months_yes)?);
            type WeekdayNames = datetime_marker_helper!(@names/weekday, $($weekdays_yes)?);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, yes);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
            type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
            type ZoneLocationsRoot = datetime_marker_helper!(@names/zone/locations_root,);
            type ZoneExemplars = datetime_marker_helper!(@names/zone/exemplar,);
            type ZoneExemplarsRoot = datetime_marker_helper!(@names/zone/exemplar_root,);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
            type ZoneStandardLong = datetime_marker_helper!(@names/zone/standard_long,);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
            type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
        }
        impl DateInputMarkers for $type_time {
            type YearInput = datetime_marker_helper!(@input/year, $($year_yes)?);
            type MonthInput = datetime_marker_helper!(@input/month, $($month_yes)?);
            type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, $($day_of_month_yes)?);
            type DayOfYearInput = datetime_marker_helper!(@input/day_of_year, $($day_of_year_yes)?);
            type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, $($day_of_week_yes)?);
        }
        impl<C: CldrCalendar> TypedDateDataMarkers<C> for $type_time {
            type DateSkeletonPatternsV1 = datetime_marker_helper!(@dates/typed, yes);
            type YearNamesV1 = datetime_marker_helper!(@years/typed, $($years_yes)?);
            type MonthNamesV1 = datetime_marker_helper!(@months/typed, $($months_yes)?);
            type WeekdayNamesV1 = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl DateDataMarkers for $type_time {
            type Skel = datetime_marker_helper!(@calmarkers, yes);
            type Year = datetime_marker_helper!(@calmarkers, $($years_yes)?);
            type Month = datetime_marker_helper!(@calmarkers, $($months_yes)?);
            type WeekdayNamesV1 = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl TimeMarkers for $type_time {
            // TODO(#6497): Consider making dayperiods optional
            type DayPeriodNamesV1 = datetime_marker_helper!(@dayperiods, yes);
            type TimeSkeletonPatternsV1 = datetime_marker_helper!(@times, yes);
            type HourInput = datetime_marker_helper!(@input/hour, yes);
            type MinuteInput = datetime_marker_helper!(@input/minute, yes);
            type SecondInput = datetime_marker_helper!(@input/second, yes);
            type NanosecondInput = datetime_marker_helper!(@input/Nanosecond, yes);
        }
        impl DateTimeMarkers for $type_time {
            type D = Self;
            type T = Self;
            type Z = ();
            type GluePatternV1 = datetime_marker_helper!(@glue, yes);
        }
        impl_composite!($type_time, DateTime, DateAndTimeFieldSet);
        impl $type_time {
            pub(crate) fn to_date_field_set(self) -> $type {
                $type {
                    length: self.length,
                    $(alignment: yes_to!(self.alignment, $option_alignment_yes),)?
                    $(year_style: yes_to!(self.year_style, $years_yes),)?
                }
            }
        }
    };
}

/// Implements a field set of calendar period fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// See [`impl_date_marker`].
macro_rules! impl_calendar_period_marker {
    (
        $(#[$attr:meta])*
        $type:ident,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        $(years = $years_yes:ident,)?
        $(months = $months_yes:ident,)?
        $(dates = $dates_yes:ident,)?
        $(input_year = $year_yes:ident,)?
        $(input_month = $month_yes:ident,)?
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_date_or_calendar_period_marker!(
            $(#[$attr])*
            $type,
            description = $description,
            sample_length = $sample_length,
            sample = $sample,
            $(years = $years_yes,)?
            $(months = $months_yes,)?
            $(dates = $dates_yes,)?
            $(input_year = $year_yes,)?
            $(input_month = $month_yes,)?
            $(input_any_calendar_kind = $any_calendar_kind_yes,)?
            $(option_alignment = $option_alignment_yes,)?
        );
        impl_composite!($type, CalendarPeriod, CalendarPeriodFieldSet);
    };
}

/// Implements a field set of time fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// Documentation for each option is shown inline below.
macro_rules! impl_time_marker {
    (
        $(#[$attr:meta])*
        // The name of the type being created.
        $type:ident,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether day periods can occur.
        $(dayperiods = $dayperiods_yes:ident,)?
        // Whether the input should include hours.
        $(input_hour = $hour_yes:ident,)?
        // Whether the input should contain minutes.
        $(input_minute = $minute_yes:ident,)?
        // Whether the input should contain seconds.
        $(input_second = $second_yes:ident,)?
        // Whether the input should contain fractional seconds.
        $(input_subsecond = $Nanosecond_yes:ident,)?
    ) => {
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
            ///
            /// # Examples
            ///
            /// ```
            /// use icu::datetime::input::Time;
            /// use icu::datetime::NoCalendarFormatter;
            #[doc = concat!("use icu::datetime::fieldsets::", stringify!($type), ";")]
            /// use icu::locale::locale;
            /// use writeable::assert_writeable_eq;
            ///
            #[doc = concat!("let fmt = NoCalendarFormatter::try_new(")]
            ///     locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let time = Time::try_new(15, 47, 50, 0).unwrap();
            ///
            /// assert_writeable_eq!(
            ///     fmt.format(&time),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            $(#[$attr])*
            $type,
            sample_length: $sample_length,
            alignment: yes,
            time_precision: yes,
        );
        impl_marker_length_constructors!(
            $type,
            alignment: yes,
            time_precision: yes,
        );
        impl_time_precision_constructors!(
            $type,
        );
        impl_zone_combo_helpers!($type, TimeZone, TimeFieldSet);
        impl UnstableSealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year,);
            type MonthNames = datetime_marker_helper!(@names/month,);
            type WeekdayNames = datetime_marker_helper!(@names/weekday,);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, $($dayperiods_yes)?);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
            type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
            type ZoneLocationsRoot = datetime_marker_helper!(@names/zone/locations_root,);
            type ZoneExemplars = datetime_marker_helper!(@names/zone/exemplar,);
            type ZoneExemplarsRoot = datetime_marker_helper!(@names/zone/exemplar_root,);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
            type ZoneStandardLong = datetime_marker_helper!(@names/zone/standard_long,);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
            type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
        }
        impl TimeMarkers for $type {
            type DayPeriodNamesV1 = datetime_marker_helper!(@dayperiods, $($dayperiods_yes)?);
            type TimeSkeletonPatternsV1 = datetime_marker_helper!(@times, yes);
            type HourInput = datetime_marker_helper!(@input/hour, $($hour_yes)?);
            type MinuteInput = datetime_marker_helper!(@input/minute, $($minute_yes)?);
            type SecondInput = datetime_marker_helper!(@input/second, $($second_yes)?);
            type NanosecondInput = datetime_marker_helper!(@input/Nanosecond, $($Nanosecond_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = ();
            type T = Self;
            type Z = ();
            type GluePatternV1 = datetime_marker_helper!(@glue,);
        }
        impl_composite!($type, Time, TimeFieldSet);
    };
}

/// Implements a field set of time zone fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// Documentation for each option is shown inline below.
macro_rules! impl_zone_marker {
    (
        $(#[$attr:meta])*
        // The name of the type being created.
        $type:ident,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the skeleton if this is the only field.
        length_override = $length_override:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // The field symbol and field length.
        field = $field:expr,
        // The type in ZoneFieldSet for this field set
        // Whether zone-essentials should be loaded.
        $(zone_essentials = $zone_essentials_yes:ident,)?
        // Whether locations names are needed.
        $(zone_locations = $zone_locations_yes:ident,)?
        // Whether exemplar city names are needed.
        $(zone_exemplars = $zone_exemplars_yes:ident,)?
        // Whether generic long names are needed.
        $(zone_generic_long = $zone_generic_long_yes:ident,)?
        // Whether generic short names are needed.
        $(zone_generic_short = $zone_generic_short_yes:ident,)?
        // Whether standard long names are needed.
        $(zone_standard_long = $zone_standard_long_yes:ident,)?
        // Whether specific long names are needed.
        $(zone_specific_long = $zone_specific_long_yes:ident,)?
        // Whether specific short names are needed.
        $(zone_specific_short = $zone_specific_short_yes:ident,)?
        // Whether metazone periods are needed
        $(metazone_periods = $metazone_periods_yes:ident,)?
        // Whether to require the TimeZone
        $(input_tzid = $tzid_input_yes:ident,)?
        // Whether to require the Timestamp
        $(input_timestamp = $timestamp_input_yes:ident,)?
    ) => {
        #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        #[doc = concat!("use icu::datetime::fieldsets::zone::", stringify!($type), ";")]
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        #[doc = concat!("    ", stringify!($type))]
        /// )
        /// .unwrap();
        ///
        /// // Time zone info for America/Chicago in the summer
        /// let zone = TimeZone(subtag!("uschi"))
        ///     .with_offset("-05".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 8, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&zone),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        $(#[$attr])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[allow(clippy::exhaustive_structs)] // singleton marker
        pub struct $type;
        impl UnstableSealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year,);
            type MonthNames = datetime_marker_helper!(@names/month,);
            type WeekdayNames = datetime_marker_helper!(@names/weekday,);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials, $($zone_essentials_yes)?);
            type ZoneLocations = datetime_marker_helper!(@names/zone/locations, $($zone_locations_yes)?);
            type ZoneLocationsRoot = datetime_marker_helper!(@names/zone/locations_root, $($zone_locations_yes)?);
            type ZoneExemplars = datetime_marker_helper!(@names/zone/exemplars, $($zone_exemplars_yes)?);
            type ZoneExemplarsRoot = datetime_marker_helper!(@names/zone/exemplars_root, $($zone_exemplars_yes)?);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long, $($zone_generic_long_yes)?);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short, $($zone_generic_short_yes)?);
            type ZoneStandardLong = datetime_marker_helper!(@names/zone/standard_long, $($zone_standard_long_yes)?);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long, $($zone_specific_long_yes)?);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short, $($zone_specific_short_yes)?);
            type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods, $($metazone_periods_yes)?);
        }
        impl ZoneMarkers for $type {
            type TimeZoneIdInput = datetime_marker_helper!(@input/timezone/id, $($tzid_input_yes)?);
            type TimeZoneOffsetInput = datetime_marker_helper!(@input/timezone/offset, yes);
            type TimeZoneNameTimestampInput = datetime_marker_helper!(@input/timezone/timestamp, $($timestamp_input_yes)?);
            type EssentialsV1 = datetime_marker_helper!(@data/zone/essentials, $($zone_essentials_yes)?);
            type LocationsV1 = datetime_marker_helper!(@data/zone/locations, $($zone_locations_yes)?);
            type LocationsRootV1 = datetime_marker_helper!(@data/zone/locations_root, $($zone_locations_yes)?);
            type ExemplarCitiesV1 = datetime_marker_helper!(@data/zone/exemplars, $($zone_exemplars_yes)?);
            type ExemplarCitiesRootV1 = datetime_marker_helper!(@data/zone/exemplars_root, $($zone_exemplars_yes)?);
            type GenericLongV1 = datetime_marker_helper!(@data/zone/generic_long, $($zone_generic_long_yes)?);
            type GenericShortV1 = datetime_marker_helper!(@data/zone/generic_short, $($zone_generic_short_yes)?);
            type StandardLongV1 = datetime_marker_helper!(@data/zone/standard_long, $($zone_standard_long_yes)?);
            type SpecificLongV1 = datetime_marker_helper!(@data/zone/specific_long, $($zone_specific_long_yes)?);
            type SpecificShortV1 = datetime_marker_helper!(@data/zone/specific_short, $($zone_specific_short_yes)?);
            type MetazonePeriodV1 = datetime_marker_helper!(@data/zone/metazone_periods, $($metazone_periods_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = ();
            type T = ();
            type Z = Self;
            type GluePatternV1 = datetime_marker_helper!(@glue,);
        }
        impl_composite!($type, Zone, ZoneFieldSet);
        impl $type {
            pub(crate) fn to_field(self) -> (fields::TimeZone, fields::FieldLength) {
                $field
            }
        }
    };
}

impl_date_marker!(
    /// This format may use ordinal formatting, such as "the 17th",
    /// in the future. See CLDR-18040.
    D,
    DT,
    description = "day of month (standalone)",
    sample_length = short,
    sample = "17",
    sample_time = "17, 3:47:50 PM",
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_date_marker!(
    E,
    ET,
    description = "weekday (standalone)",
    sample_length = long,
    sample = "Friday",
    sample_time = "Friday 3:47:50 PM",
    weekdays = yes,
    input_day_of_week = yes,
);

impl_date_marker!(
    /// This format may use ordinal formatting, such as "Friday the 17th",
    /// in the future. See CLDR-18040.
    DE,
    DET,
    description = "day of month and weekday",
    sample_length = long,
    sample = "17 Friday",
    sample_time = "17 Friday, 3:47:50 PM",
    weekdays = yes,
    input_day_of_month = yes,
    input_day_of_week = yes,
    option_alignment = yes,
);

impl_date_marker!(
    MD,
    MDT,
    description = "month and day",
    sample_length = medium,
    sample = "May 17",
    sample_time = "May 17, 3:47:50 PM",
    months = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_date_marker!(
    /// See CLDR-18040 for progress on improving this format.
    MDE,
    MDET,
    description = "month, day, and weekday",
    sample_length = medium,
    sample = "Fri, May 17",
    sample_time = "Fri, May 17, 3:47:50 PM",
    months = yes,
    weekdays = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_day_of_week = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_date_marker!(
    YMD,
    YMDT,
    description = "year, month, and day",
    sample_length = short,
    sample = "5/17/24",
    sample_time = "5/17/24, 3:47:50 PM",
    years = yes,
    months = yes,
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_date_marker!(
    YMDE,
    YMDET,
    description = "year, month, day, and weekday",
    sample_length = short,
    sample = "Fri, 5/17/24",
    sample_time = "Fri, 5/17/24, 3:47:50 PM",
    years = yes,
    months = yes,
    weekdays = yes,
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_day_of_week = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_calendar_period_marker!(
    Y,
    description = "year (standalone)",
    sample_length = medium,
    sample = "2024",
    years = yes,
    input_year = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_calendar_period_marker!(
    M,
    description = "month (standalone)",
    sample_length = long,
    sample = "May",
    months = yes,
    input_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_calendar_period_marker!(
    YM,
    description = "year and month",
    sample_length = medium,
    sample = "May 2024",
    years = yes,
    months = yes,
    input_year = yes,
    input_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_time_marker!(
    /// Hours can be switched between 12-hour and 24-hour time via the `u-hc` locale keyword
    /// or [`DateTimeFormatterPreferences`].
    ///
    /// ```
    /// use icu::datetime::input::Time;
    /// use icu::datetime::fieldsets::T;
    /// use icu::datetime::NoCalendarFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // By default, en-US uses 12-hour time and fr-FR uses 24-hour time,
    /// // but we can set overrides.
    ///
    /// let formatter = NoCalendarFormatter::try_new(
    ///     locale!("en-US-u-hc-h12").into(),
    ///     T::hm(),
    /// )
    /// .unwrap();
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
    ///     "4:12 PM"
    /// );
    ///
    /// let formatter = NoCalendarFormatter::try_new(
    ///     locale!("en-US-u-hc-h23").into(),
    ///     T::hm(),
    /// )
    /// .unwrap();
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
    ///     "16:12"
    /// );
    ///
    /// let formatter = NoCalendarFormatter::try_new(
    ///     locale!("fr-FR-u-hc-h12").into(),
    ///     T::hm(),
    /// )
    /// .unwrap();
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
    ///     "4:12 PM"
    /// );
    ///
    /// let formatter = NoCalendarFormatter::try_new(
    ///     locale!("fr-FR-u-hc-h23").into(),
    ///     T::hm(),
    /// )
    /// .unwrap();
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
    ///     "16:12"
    /// );
    /// ```
    ///
    /// Hour cycles `h11` and `h24` are supported, too:
    ///
    /// ```
    /// use icu::datetime::input::Time;
    /// use icu::datetime::fieldsets::T;
    /// use icu::datetime::NoCalendarFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = NoCalendarFormatter::try_new(
    ///     locale!("und-u-hc-h11").into(),
    ///     T::hm(),
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(0, 0, 0, 0).unwrap()),
    ///     "0:00 AM"
    /// );
    ///
    /// let formatter = NoCalendarFormatter::try_new(
    ///     locale!("und-u-hc-h23").into(),
    ///     T::hm(),
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(0, 0, 0, 0).unwrap()),
    ///     "00:00"
    /// );
    /// ```
    ///
    /// Conveniently construct a time formatter with subseconds:
    ///
    /// ```
    /// use icu::datetime::input::Time;
    /// use icu::datetime::fieldsets::T;
    /// use icu::datetime::options::SubsecondDigits;
    /// use icu::datetime::NoCalendarFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = NoCalendarFormatter::try_new(
    ///     locale!("en").into(),
    ///     T::hmss(SubsecondDigits::S4),
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(18, 24, 36, 987654321).unwrap()),
    ///     "6:24:36.9876 PM"
    /// );
    /// ```
    ///
    /// [`DateTimeFormatterPreferences`]: crate::DateTimeFormatterPreferences
    T,
    description = "time (locale-dependent hour cycle)",
    sample_length = medium,
    sample = "3:47:50 PM",
    dayperiods = yes,
    input_hour = yes,
    input_minute = yes,
    input_second = yes,
    input_subsecond = yes,
);

/// Time zone field sets
pub mod zone {
    use super::*;
    impl_zone_marker!(
        /// If a non-location name is not defined, falls back to the specific location format:
        ///
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::SpecificLong;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for Europe/Istanbul in the winter
        /// let zone = TimeZone(subtag!("trist"))
        ///     .with_offset("+03".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 1, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     SpecificLong,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&zone),
        ///     "Türkiye Standard Time"
        /// );
        /// ```
        ///
        /// If the offset doesn't match one of the valid offsets for the zone, falls back to the localized offset format:
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::SpecificLong;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for America/Chicago with a wrong offset
        /// let wrong_offset = TimeZone(subtag!("uschi"))
        ///     .with_offset("-04".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 8, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     SpecificLong,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&wrong_offset),
        ///     "GMT-04:00"
        /// );
        ///
        /// // This of course always happens for the unknown time zone
        /// let unk = TimeZone::UNKNOWN.with_offset(wrong_offset.offset()).with_zone_name_timestamp(wrong_offset.zone_name_timestamp());
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&unk),
        ///     "GMT-04:00"
        /// );
        /// ```
        ///
        /// Since non-location names might change over time,
        /// this time zone style requires a reference time.
        ///
        /// ```compile_fail,E0271
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::SpecificLong;
        /// use icu::locale::{locale, subtags::subtag};
        /// use icu::datetime::input::{TimeZone, UtcOffset};
        ///
        /// let time_zone_basic = TimeZone(subtag!("uschi")).without_offset();
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     SpecificLong,
        /// )
        /// .unwrap();
        ///
        /// // error[E0271]: type mismatch resolving `<Base as TimeZoneModel>::ZoneNameTimestamp == ZoneNameTimestamp`
        /// formatter.format(&time_zone_basic);
        /// ```
        SpecificLong,
        description = "time zone in specific non-location format, long length",
        length_override = Long,
        sample = "Central Daylight Time",
        field = (fields::TimeZone::SpecificNonLocation, fields::FieldLength::Four),
        zone_essentials = yes,
        zone_locations = yes,
        zone_standard_long = yes,
        zone_specific_long = yes,
        metazone_periods = yes,
        input_tzid = yes,
        input_timestamp = yes,
    );

    impl_zone_marker!(
        /// If a non-location name is not defined, falls back to the localized offset format.
        /// This is the case for most zones, as each locale only contains the short names that are
        /// used locally and generally understood:
        ///
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::SpecificShort;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for Asia/Tokyo
        /// let zone = TimeZone(subtag!("jptyo"))
        ///     .with_offset("+09".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 1, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     SpecificShort,
        /// )
        /// .unwrap();
        ///
        /// // "JST" is not generally known in American English
        /// assert_writeable_eq!(
        ///     fmt.format(&zone),
        ///     "GMT+9"
        /// );
        /// ```
        ///
        /// If the offset doesn't match one of the valid offsets for the zone, falls back to the localized offset format:
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::SpecificShort;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for America/Chicago with a wrong offset
        /// let wrong_offset = TimeZone(subtag!("uschi"))
        ///     .with_offset("-04".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 8, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     SpecificShort,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&wrong_offset),
        ///     "GMT-4"
        /// );
        ///
        /// // This of course always happens for the unknown time zone
        /// let unk = TimeZone::UNKNOWN.with_offset(wrong_offset.offset()).with_zone_name_timestamp(wrong_offset.zone_name_timestamp());
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&unk),
        ///     "GMT-4"
        /// );
        /// ```
        ///
        /// Since non-location names might change over time,
        /// this time zone style requires a reference time.
        ///
        /// ```compile_fail,E0271
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::SpecificShort;
        /// use icu::datetime::input::TimeZone;
        /// use icu::locale::{locale, subtags::subtag};
        ///
        /// let time_zone_basic = TimeZone(subtag!("uschi")).without_offset();
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     SpecificShort,
        /// )
        /// .unwrap();
        ///
        /// // error[E0271]: type mismatch resolving `<Base as TimeZoneModel>::ZoneNameTimestamp == ZoneNameTimestamp`
        /// formatter.format(&time_zone_basic);
        /// ```
        SpecificShort,
        description = "time zone in specific non-location format, short length",
        length_override = Short,
        sample = "CDT",
        field = (fields::TimeZone::SpecificNonLocation, fields::FieldLength::One),
        zone_essentials = yes,
        zone_specific_short = yes,
        metazone_periods = yes,
        input_tzid = yes,
        input_timestamp = yes,
    );

    impl_zone_marker!(
        /// All shapes of time zones can be formatted with this style.
        ///
        /// ```
        /// use icu::datetime::input::Date;
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::LocalizedOffsetLong;
        /// use icu::datetime::input::{DateTime, Time, TimeZone, UtcOffset};
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// let utc_offset = "-06".parse().unwrap();
        /// let time_zone_basic = TimeZone(subtag!("uschi")).with_offset(Some(utc_offset));
        ///
        /// let date = Date::try_new_iso(2024, 10, 18).unwrap();
        /// let time = Time::start_of_day();
        /// let time_zone_at_time = time_zone_basic.at_date_time_iso(DateTime{ date, time });
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     LocalizedOffsetLong,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     formatter.format(&utc_offset),
        ///     "GMT-06:00"
        /// );
        ///
        /// assert_writeable_eq!(
        ///     formatter.format(&time_zone_basic),
        ///     "GMT-06:00"
        /// );
        ///
        /// assert_writeable_eq!(
        ///     formatter.format(&time_zone_at_time),
        ///     "GMT-06:00"
        /// );
        /// ```
        LocalizedOffsetLong,
        description = "UTC offset, long length",
        length_override = Long,
        sample = "GMT-05:00",
        field = (fields::TimeZone::LocalizedOffset, fields::FieldLength::Four),
        zone_essentials = yes,
    );

    impl_zone_marker!(
        /// All shapes of time zones can be formatted with this style.
        ///
        /// ```
        /// use icu::datetime::input::Date;
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::LocalizedOffsetShort;
        /// use icu::datetime::input::{DateTime, Time, TimeZone, UtcOffset};
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// let utc_offset = "-06".parse().unwrap();
        /// let time_zone_basic = TimeZone(subtag!("uschi")).with_offset(Some(utc_offset));
        ///
        /// let date = Date::try_new_iso(2024, 10, 18).unwrap();
        /// let time = Time::start_of_day();
        /// let time_zone_at_time = time_zone_basic.at_date_time_iso(DateTime{ date, time });
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     LocalizedOffsetShort,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     formatter.format(&utc_offset),
        ///     "GMT-6"
        /// );
        ///
        /// assert_writeable_eq!(
        ///     formatter.format(&time_zone_basic),
        ///     "GMT-6"
        /// );
        ///
        /// assert_writeable_eq!(
        ///     formatter.format(&time_zone_at_time),
        ///     "GMT-6"
        /// );
        /// ```
        LocalizedOffsetShort,
        description = "UTC offset, short length",
        length_override = Short,
        sample = "GMT-5",
        field = (fields::TimeZone::LocalizedOffset, fields::FieldLength::One),
        zone_essentials = yes,
    );

    impl_zone_marker!(
        /// If a non-location name is not defined, falls back to the specific location format:
        ///
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::GenericLong;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for Europe/Istanbul in the winter
        /// let zone = TimeZone(subtag!("trist"))
        ///     .with_offset("+03".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 1, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     GenericLong,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&zone),
        ///     "Türkiye Time"
        /// );
        /// ```
        ///
        /// If the offset doesn't match one of the valid offsets for the zone, falls back to the localized offset format:
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::GenericLong;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for America/Chicago with a wrong offset
        /// let wrong_offset = TimeZone(subtag!("uschi"))
        ///     .with_offset("-04".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 8, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     GenericLong,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&wrong_offset),
        ///     "GMT-04:00"
        /// );
        ///
        /// // This of course always happens for the unknown time zone
        /// let unk = TimeZone::UNKNOWN.with_offset(wrong_offset.offset()).with_zone_name_timestamp(wrong_offset.zone_name_timestamp());
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&unk),
        ///     "GMT-04:00"
        /// );
        /// ```
        ///
        /// Since non-location names might change over time,
        /// this time zone style requires a reference time.
        ///
        /// ```compile_fail,E0271
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::GenericLong;
        /// use icu::datetime::input::TimeZone;
        /// use icu::locale::{locale, subtags::subtag};
        ///
        /// let time_zone_basic = TimeZone(subtag!("uschi")).without_offset();
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     GenericLong,
        /// )
        /// .unwrap();
        ///
        /// // error[E0271]: type mismatch resolving `<Base as TimeZoneModel>::ZoneNameTimestamp == ZoneNameTimestamp`
        /// formatter.format(&time_zone_basic);
        /// ```
        GenericLong,
        description = "time zone in generic non-location format, long length",
        length_override = Long,
        sample = "Central Time",
        field = (fields::TimeZone::GenericNonLocation, fields::FieldLength::Four),
        zone_essentials = yes,
        zone_locations = yes,
        zone_generic_long = yes,
        zone_standard_long = yes,
        metazone_periods = yes,
        input_tzid = yes,
        input_timestamp = yes,
    );

    impl_zone_marker!(
        /// If a non-location name is not defined, falls back to the location format, which is
        /// often significantly longer!
        /// This is the case for most zones, as each locale only contains the short names that are
        /// used locally and generally understood:
        ///
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::GenericShort;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for Asia/Tokyo
        /// let zone = TimeZone(subtag!("jptyo"))
        ///     .with_offset("+09".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 1, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     GenericShort,
        /// )
        /// .unwrap();
        ///
        /// // "JST" is not generally known in American English
        /// assert_writeable_eq!(
        ///     fmt.format(&zone),
        ///     "Japan Time"
        /// );
        /// ```
        ///
        /// If the offset doesn't match one of the valid offsets for the zone, falls back to the localized offset format:
        /// ```
        /// use icu::datetime::input::{Date, DateTime, Time, TimeZone, TimeZoneInfo, UtcOffset};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::GenericShort;
        /// use icu::locale::{locale, subtags::subtag};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Time zone info for America/Chicago with a wrong offset
        /// let wrong_offset = TimeZone(subtag!("uschi"))
        ///     .with_offset("-04".parse().ok())
        ///     .at_date_time_iso(DateTime{ date: Date::try_new_iso(2022, 8, 29).unwrap(), time: Time::start_of_day() });
        ///
        /// let fmt = NoCalendarFormatter::try_new(
        ///     locale!("en").into(),
        ///     GenericShort,
        /// )
        /// .unwrap();
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&wrong_offset),
        ///     "GMT-4"
        /// );
        ///
        /// // This of course always happens for the unknown time zone
        /// let unk = TimeZone::UNKNOWN.with_offset(wrong_offset.offset()).with_zone_name_timestamp(wrong_offset.zone_name_timestamp());
        ///
        /// assert_writeable_eq!(
        ///     fmt.format(&unk),
        ///     "GMT-4"
        /// );
        /// ```
        ///
        /// Since non-location names might change over time,
        /// this time zone style requires a reference time.
        ///
        /// ```compile_fail,E0271
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::GenericShort;
        /// use icu::datetime::input::TimeZone;
        /// use icu::locale::{locale, subtags::subtag};
        ///
        /// let time_zone_basic = TimeZone(subtag!("uschi")).without_offset();
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     GenericShort,
        /// )
        /// .unwrap();
        ///
        /// // error[E0271]: type mismatch resolving `<Base as TimeZoneModel>::ZoneNameTimestamp == ZoneNameTimestamp`
        /// formatter.format(&time_zone_basic);
        /// ```
        GenericShort,
        description = "time zone in generic non-location format, short length",
        length_override = Short,
        sample = "CT",
        field = (fields::TimeZone::GenericNonLocation, fields::FieldLength::One),
        zone_essentials = yes,
        zone_locations = yes,
        zone_generic_short = yes,
        metazone_periods = yes,
        input_tzid = yes,
        input_timestamp = yes,
    );

    impl_zone_marker!(
        /// A time zone ID is required to format with this style.
        /// For example, a raw [`UtcOffset`] cannot be used here.
        ///
        /// ```compile_fail,E0277
        /// use icu::datetime::input::{DateTime, Iso};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::Location;
        /// use icu::datetime::input::UtcOffset;
        /// use tinystr::tinystr;
        /// use icu::locale::locale;
        /// use writeable::assert_writeable_eq;
        ///
        /// let utc_offset = UtcOffset::try_from_str("-06").unwrap();
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     Location,
        /// )
        /// .unwrap();
        ///
        /// // error[E0277]: the trait bound `UtcOffset: AllInputMarkers<Location>` is not satisfied
        /// // note: required by a bound in `NoCalendarFormatter::<C, FSet>::format`
        /// formatter.format(&utc_offset);
        /// ```
        Location,
        description = "time zone in location format",
        length_override = Long,
        sample = "Chicago Time",
        field = (fields::TimeZone::Location, fields::FieldLength::Four),
        zone_essentials = yes,
        zone_locations = yes,
        input_tzid = yes,
    );

    impl_zone_marker!(
        /// A time zone ID is required to format with this style.
        /// For example, a raw [`UtcOffset`] cannot be used here.
        ///
        /// ```compile_fail,E0277
        /// use icu::datetime::input::{DateTime, Iso};
        /// use icu::datetime::NoCalendarFormatter;
        /// use icu::datetime::fieldsets::zone::ExemplarCity;
        /// use icu::datetime::input::UtcOffset;
        /// use tinystr::tinystr;
        /// use icu::locale::locale;
        /// use writeable::assert_writeable_eq;
        ///
        /// let utc_offset = UtcOffset::try_from_str("-06").unwrap();
        ///
        /// let formatter = NoCalendarFormatter::try_new(
        ///     locale!("en-US").into(),
        ///     ExemplarCity,
        /// )
        /// .unwrap();
        ///
        /// // error[E0277]: the trait bound `UtcOffset: AllInputMarkers<ExemplarCity>` is not satisfied
        /// // note: required by a bound in `NoCalendarFormatter::<C, FSet>::format`
        /// formatter.format(&utc_offset);
        /// ```
        ExemplarCity,
        description = "time zone in exemplar city format",
        length_override = Long,
        sample = "Chicago",
        field = (fields::TimeZone::Location, fields::FieldLength::Three),
        zone_locations = yes,
        zone_exemplars = yes,
        input_tzid = yes,
    );
}

impl_zone_combo_helpers!(DateFieldSet, DateZone, DateFieldSet);

impl_zone_combo_helpers!(TimeFieldSet, TimeZone, TimeFieldSet);

impl_zone_combo_helpers!(DateAndTimeFieldSet, DateTimeZone, DateAndTimeFieldSet);
