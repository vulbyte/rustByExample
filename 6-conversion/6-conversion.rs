//primitive types can be converted to each other through casting
//
//rust addresses conversion between custom types (ie, struct and enum) by the use of traits. the
//generic conversions will the use `From` and `Into` Traits. howver there are more specific ones
//for the common cases, in particular when converting to and from `String`s
