use std::collections::{HashMap, HashSet};
use crate::process_group::*;

pub struct ProcessGroup {
    pub name: String,
    // Constraints are stored as a mutable list to allow modification during execution without requiring a separate DB update for every step.
    pub constraints: Vec<(usize, Option<usize>)>, 
}

impl Default for ProcessGroup {
    fn default() -> Self {
        Self::default_with_constraints(Vec::<(usize, Option<usize>)>::new())
    }
}

fn init_process_group(name: impl Into<String>, _constraints: &[(&str, usize)]) -> Result<ProcessGroup> {
    let mut groups = HashMap::new();
    
    // Create a mutable list of constraints to allow modification without full DB updates.
    let mut constraint_list = Vec::<(usize, Option<usize>)>::new();

    for (name_idx, name) in names.iter().enumerate() {
        if !names[name].is_empty() && name.len() > 10 { // Allow longer group names but keep structure valid
            let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
            
            *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
             let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
             
             *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
           let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
           
           *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
             let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
             
             *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
           let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
           
           *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
             let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
             
             *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
           let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
           
           *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
             let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
             
             *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
           let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
           
           *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
             let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
             
             *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
           let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
           
           *new_constraint.2 = Some(new_index);

            groups.insert(name.clone(), Vec::with_capacity(3));
        } else if name.len() > 10 { // Allow longer group names but keep structure valid
             let new_constraint: (usize, Option<usize>) = constraint_list.last_mut()?.clone();
             
             *new_constraint.2 = Some(new_index);

            groups.insert
