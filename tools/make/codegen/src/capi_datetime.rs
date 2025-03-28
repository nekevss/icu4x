// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{fs::File, path::PathBuf};

use askama::Template;
use icu::datetime::fieldsets::builder::*;

const GENERATED_BY: &str = std::concat!("@generated by ", std::file!());

#[derive(Debug)]
struct ConsumedOptions {
    length: bool,
    alignment: bool,
    year_style: bool,
}

impl ConsumedOptions {
    fn from_builder(builder: FieldSetBuilder) -> Option<Self> {
        match builder.build_composite() {
            Ok(_) => Some(ConsumedOptions {
                length: true,
                alignment: true,
                year_style: true,
            }),
            Err(BuilderError::SuperfluousOptions(options)) => Some(ConsumedOptions {
                length: options.length.is_none(),
                alignment: options.alignment.is_none(),
                year_style: options.year_style.is_none(),
            }),
            Err(BuilderError::InvalidDateFields) => None,
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}

#[derive(Copy, Clone)]
enum DateOrTime {
    Date,
    Time,
    #[allow(dead_code)] // temporary
    DateTime,
}

impl DateOrTime {
    pub fn camel(self) -> &'static str {
        match self {
            DateOrTime::Date => "Date",
            DateOrTime::Time => "Time",
            DateOrTime::DateTime => "DateTime",
        }
    }
    pub fn lower(self) -> &'static str {
        match self {
            DateOrTime::Date => "date",
            DateOrTime::Time => "time",
            DateOrTime::DateTime => "datetime",
        }
    }
    pub fn formatter_kinds(self) -> &'static [FormatterKind] {
        match self {
            DateOrTime::Date | DateOrTime::DateTime => &[
                FormatterKind {
                    is_fixed_calendar: false,
                    is_gregorian: false,
                },
                FormatterKind {
                    is_fixed_calendar: true,
                    is_gregorian: true,
                },
            ],
            DateOrTime::Time => &[FormatterKind {
                is_fixed_calendar: true,
                is_gregorian: false,
            }],
        }
    }
    pub fn field_set(self) -> &'static str {
        match self {
            DateOrTime::Date => "DateFieldSet",
            DateOrTime::Time => "TimeFieldSet",
            DateOrTime::DateTime => "CompositeDateTimeFieldSet",
        }
    }
    pub fn has_date(self) -> bool {
        matches!(self, DateOrTime::Date | DateOrTime::DateTime)
    }
    pub fn has_time(self) -> bool {
        matches!(self, DateOrTime::Time | DateOrTime::DateTime)
    }
}

#[derive(Copy, Clone)]
struct FormatterKind {
    pub is_fixed_calendar: bool,
    pub is_gregorian: bool,
}

impl FormatterKind {
    pub fn rust_type(self) -> &'static str {
        match self.is_fixed_calendar {
            true => "FixedCalendarDateTimeFormatter",
            false => "DateTimeFormatter",
        }
    }
}

#[derive(Template)]
#[template(path = "datetime_formatter.rs.jinja")]
struct DateTimeFormatterTemplate {
    flavor: DateOrTime,
    variants: Vec<DateTimeFormatterVariant>,
}

struct DateTimeFormatterVariant {
    inner: DateTimeFormatterVariantInner,
    consumed_options: ConsumedOptions,
}

enum DateTimeFormatterVariantInner {
    Date(DateFields),
    Time,
    #[allow(dead_code)] // temporary
    DateTime(DateFields),
}

impl DateTimeFormatterVariant {
    pub fn name_upper(&self) -> &'static str {
        use DateTimeFormatterVariantInner as Inner;
        match self.inner {
            Inner::Date(DateFields::D) => "D",
            Inner::Date(DateFields::MD) => "MD",
            Inner::Date(DateFields::YMD) => "YMD",
            Inner::Date(DateFields::DE) => "DE",
            Inner::Date(DateFields::MDE) => "MDE",
            Inner::Date(DateFields::YMDE) => "YMDE",
            Inner::Date(DateFields::E) => "E",
            Inner::Date(DateFields::M) => "M",
            Inner::Date(DateFields::YM) => "YM",
            Inner::Date(DateFields::Y) => "Y",
            Inner::Time => "T",
            Inner::DateTime(DateFields::D) => "DT",
            Inner::DateTime(DateFields::MD) => "MDT",
            Inner::DateTime(DateFields::YMD) => "YMDT",
            Inner::DateTime(DateFields::DE) => "DET",
            Inner::DateTime(DateFields::MDE) => "MDET",
            Inner::DateTime(DateFields::YMDE) => "YMDET",
            Inner::DateTime(DateFields::E) => "ET",
            _ => unreachable!("unknown variant"),
        }
    }
    pub fn name_lower(&self) -> &'static str {
        use DateTimeFormatterVariantInner as Inner;
        match self.inner {
            Inner::Date(DateFields::D) => "d",
            Inner::Date(DateFields::MD) => "md",
            Inner::Date(DateFields::YMD) => "ymd",
            Inner::Date(DateFields::DE) => "de",
            Inner::Date(DateFields::MDE) => "mde",
            Inner::Date(DateFields::YMDE) => "ymde",
            Inner::Date(DateFields::E) => "e",
            Inner::Date(DateFields::M) => "m",
            Inner::Date(DateFields::YM) => "ym",
            Inner::Date(DateFields::Y) => "y",
            Inner::Time => "t",
            Inner::DateTime(DateFields::D) => "dt",
            Inner::DateTime(DateFields::MD) => "mdt",
            Inner::DateTime(DateFields::YMD) => "ymdt",
            Inner::DateTime(DateFields::DE) => "det",
            Inner::DateTime(DateFields::MDE) => "mdet",
            Inner::DateTime(DateFields::YMDE) => "ymdet",
            Inner::DateTime(DateFields::E) => "et",
            _ => unreachable!("unknown variant"),
        }
    }
    pub fn is_default_constructor(&self) -> bool {
        use DateTimeFormatterVariantInner as Inner;
        matches!(
            self.inner,
            Inner::Date(DateFields::YMD) | Inner::Time | Inner::DateTime(DateFields::YMD)
        )
    }
    pub fn is_only_constructor(&self) -> bool {
        use DateTimeFormatterVariantInner as Inner;
        matches!(self.inner, Inner::Time)
    }
}

#[derive(Template)]
#[template(path = "zoned_formatter.rs.jinja")]
struct ZonedFormatterTemplate {
    flavor: DateOrTime,
    variants: Vec<ZonedFormatterVariant>,
}

struct ZonedFormatterVariant {
    zone_style: ZoneStyle,
}

impl ZonedFormatterVariant {
    pub fn name_lower(&self) -> &str {
        match self.zone_style {
            ZoneStyle::SpecificLong => "specific_long",
            ZoneStyle::SpecificShort => "specific_short",
            ZoneStyle::LocalizedOffsetLong => "localized_offset_long",
            ZoneStyle::LocalizedOffsetShort => "localized_offset_short",
            ZoneStyle::GenericLong => "generic_long",
            ZoneStyle::GenericShort => "generic_short",
            ZoneStyle::Location => "location",
            ZoneStyle::ExemplarCity => "exemplar_city",
            _ => unreachable!("unknown variant"),
        }
    }
    pub fn name_camel(&self) -> &str {
        match self.zone_style {
            ZoneStyle::SpecificLong => "SpecificLong",
            ZoneStyle::SpecificShort => "SpecificShort",
            ZoneStyle::LocalizedOffsetLong => "LocalizedOffsetLong",
            ZoneStyle::LocalizedOffsetShort => "LocalizedOffsetShort",
            ZoneStyle::GenericLong => "GenericLong",
            ZoneStyle::GenericShort => "GenericShort",
            ZoneStyle::Location => "Location",
            ZoneStyle::ExemplarCity => "ExemplarCity",
            _ => unreachable!("unknown variant"),
        }
    }
    pub fn load_fn(&self) -> &str {
        match self.zone_style {
            ZoneStyle::SpecificLong => "specific_long_names_with_fallback",
            ZoneStyle::SpecificShort => "specific_short_names_with_fallback",
            ZoneStyle::LocalizedOffsetLong => "localized_offset_names_with_fallback",
            ZoneStyle::LocalizedOffsetShort => "localized_offset_names_with_fallback",
            ZoneStyle::GenericLong => "generic_long_names_with_fallback",
            ZoneStyle::GenericShort => "generic_short_names_with_fallback",
            ZoneStyle::Location => "location_names",
            ZoneStyle::ExemplarCity => "exemplar_city_names",
            _ => unreachable!("unknown variant"),
        }
    }
    pub fn is_default_constructor(&self) -> bool {
        matches!(self.zone_style, ZoneStyle::GenericShort)
    }
}

pub fn main() {
    let mut date_formatter_template = DateTimeFormatterTemplate {
        flavor: DateOrTime::Date,
        variants: Vec::new(),
    };
    let mut time_formatter_template = DateTimeFormatterTemplate {
        flavor: DateOrTime::Time,
        variants: Vec::new(),
    };
    let mut zoned_date_formatter_template = ZonedFormatterTemplate {
        flavor: DateOrTime::Date,
        variants: Vec::new(),
    };

    for date_fields in DateFields::VALUES.iter() {
        // Determine the options for these date fields
        let mut builder = FieldSetBuilder::new();
        builder.date_fields = Some(*date_fields);
        builder.length = Some(Default::default());
        builder.alignment = Some(Default::default());
        builder.year_style = Some(Default::default());

        let consumed_options = ConsumedOptions::from_builder(builder.clone()).unwrap();
        println!("{date_fields:?} as Date => {consumed_options:?}");
        assert!(consumed_options.length); // all constructors accept a length
        date_formatter_template
            .variants
            .push(DateTimeFormatterVariant {
                inner: DateTimeFormatterVariantInner::Date(*date_fields),
                consumed_options,
            });

        builder.time_precision = Some(Default::default());
        let consumed_options = ConsumedOptions::from_builder(builder.clone());
        println!("{date_fields:?} as DateTime => {consumed_options:?}");

        builder.zone_style = Some(ZoneStyle::LocalizedOffsetShort);
        let consumed_options = ConsumedOptions::from_builder(builder.clone());
        println!("{date_fields:?} as DateTimeZone => {consumed_options:?}");

        builder.time_precision = None;
        let consumed_options = ConsumedOptions::from_builder(builder.clone());
        println!("{date_fields:?} as DateZone => {consumed_options:?}");
    }

    for zone_style in ZoneStyle::VALUES.iter() {
        zoned_date_formatter_template
            .variants
            .push(ZonedFormatterVariant {
                zone_style: *zone_style,
            });
    }

    {
        let mut builder = FieldSetBuilder::new();
        builder.time_precision = Some(Default::default());
        builder.length = Some(Default::default());
        builder.alignment = Some(Default::default());
        builder.year_style = Some(Default::default());
        let consumed_options = ConsumedOptions::from_builder(builder.clone()).unwrap();
        time_formatter_template
            .variants
            .push(DateTimeFormatterVariant {
                inner: DateTimeFormatterVariantInner::Time,
                consumed_options,
            });
    }

    let mut path_buf = PathBuf::new();
    path_buf.push(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("../../../ffi/capi/src");

    {
        let mut path_buf = path_buf.clone();
        path_buf.push("date_formatter.rs");
        let mut file = File::create(&path_buf).unwrap();
        use std::io::Write;
        writeln!(&mut file, "{}", date_formatter_template).unwrap();
    }

    {
        let mut path_buf = path_buf.clone();
        path_buf.push("time_formatter.rs");
        let mut file = File::create(&path_buf).unwrap();
        use std::io::Write;
        writeln!(&mut file, "{}", time_formatter_template).unwrap();
    }

    {
        let mut path_buf = path_buf.clone();
        path_buf.push("zoned_date_formatter.rs");
        let mut file = File::create(&path_buf).unwrap();
        use std::io::Write;
        writeln!(&mut file, "{}", zoned_date_formatter_template).unwrap();
    }
}
