/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

//#[derive(Partial)]

pub struct Example {
    food: f64,
    bard: Option<String>,

    // #[ignorePartial(default => 42)]
    // without an default, we cannot provide a `build`. unless the ignored value type is 'default'
    something_special: u32,

    // #[forcePartial(default => 5)] // we cannot provide a Default trait for the partial, unless
    // default provided, or value type is Default
    id: u32,
    /* TODO: partialonly. Data that is present on the Partial but not on the original `Example`
     * perhabs do this by allowing a derive on the Partial */
}
impl Example {
    pub fn use_all_vars(&self) -> String {
        format!("{}{}", self.id, self.something_special)
    }
}

////// generated code starts here
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//TODO: display
// Default: Default is available when no attributes are forced (`forcePartial`) or the forced values
// have a default. TODO: Users can customize the Derives
#[allow(clippy::option_option)]
pub struct PartialExample {
    pub food: Option<f64>,
    pub bard: Option<Option<String>>,
    id: u32, //forced, so not optional
}

impl PartialExample {
    /// Generates a `Partial` by requiring the `forced` values.
    pub fn new(id: u32) -> PartialExample {
        // generated if we have forced values.
        PartialExample { id,
                         food: None,
                         bard: None }
    }

    pub fn with_arguments<FOOD, BARD>(id: u32, food: FOOD, bard: BARD) -> PartialExample
        where FOOD: std::convert::Into<Option<f64>>,
              BARD: std::convert::Into<Option<Option<String>>> {
        PartialExample { id,
                         food: food.into(),
                         bard: bard.into() }
    }

    /// add all attributes set here in to the given Partial
    ///
    /// If an attribute in here is already set in `obj`, then it gets
    /// overwritten
    pub fn merge_into(&self, obj: &mut PartialExample) {
        //for each attribute
        if let Some(food) = &self.food {
            obj.food = Some(*food);
        }
        if let Some(bard) = &self.bard {
            obj.bard = Some(bard.clone());
        }
    }

    /// check all attributes that are present on BOTH objects, if they are
    /// equal. If there are no attributes present in both, will return true.
    pub fn is_partial_equal_existing(&self, obj: &PartialExample) -> bool {
        //for each attribute
        if let Some(food) = &self.food {
            if let Some(otherfood) = &obj.food {
                if (*food - *otherfood).abs() > std::f64::EPSILON {
                    // special comparison for f32 and f64
                    return false;
                }
            }
        }

        if let Some(bard) = &self.bard {
            if let Some(otherbard) = &obj.bard {
                if *bard != *otherbard {
                    return false;
                }
            }
        }

        true
    }

    pub fn build(&self) -> Result<Example, ()> {
        if let Some(food) = &self.food {
            if let Some(bard) = &self.bard {
                return Ok(Example { food: *food,
                                    bard: bard.clone(),
                                    id: self.id,
                                    something_special: 42, });
            }
        }
        Err(())
    }
}

impl Base<()> for PartialExample {
    type Target = Example;

    const MAX_FIELDS: usize = 2;

    fn is_complete(&self) -> bool {
        //for each attribute
        self.food.is_some() && self.bard.is_some()
    }

    fn is_empty(&self) -> bool {
        self.food.is_none() && self.bard.is_none()
    }

    /// Counts the number of set attributes.
    ///
    /// Forced values do not count, as they are always present
    fn count(&self) -> u32 {
        let mut count = 0;
        //for each attribute
        if self.food.is_some() {
            count += 1;
        }

        if self.bard.is_some() {
            count += 1;
        }

        count
    }

    fn apply(&self, obj: &mut Self::Target) {
        //for each attribute
        if let Some(food) = &self.food {
            obj.food = *food;
        }
        if let Some(bard) = &self.bard {
            // clone because 'Option' does not implement Copy
            obj.bard = bard.clone();
        }
    }
}


use derive_patch::{mismatch::IncompleteError, traits::Base};

impl std::convert::TryFrom<PartialExample> for Example {
    type Error = IncompleteError<PartialExample>;

    // This function requires all `ignorePartial` values to have a default.
    fn try_from(value: PartialExample) -> Result<Self, Self::Error> {
        if value.is_complete() {
            Ok(Example { id:   value.id,
                         bard: value.bard.unwrap(),
                         food: value.food.unwrap(),

                         something_special: 42, })
        } else {
            Err(Self::Error::new("try_from",
                                 value.id.to_string(), /* todo: combine
                                                        * all ids! */
                                 value))
        }
    }
}
