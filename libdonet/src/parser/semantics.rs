/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

//! The DC parser outputs an [`Abstract Syntax Tree`], which is just a big
//! nested structure that defines the declarations in the DC file. At runtime,
//! the Donet daemon (and its services) need a class hierarchy structure in
//! memory to access while processing network messages.
//!
//! This source file defines the process of taking in the DC file abstract
//! syntax tree as input and generating an output of a class hierarchy structure,
//! where each class has pointers to its children, and vice versa, with methods
//! that make it easy for the Donet daemon to look up information on the DC contract
//! at runtime in order to understand the network messages it receives.
//!
//! [`Abstract Syntax Tree`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

use super::ast;
use super::PipelineData;
use crate::dcfile;
use crate::globals::ParseError;

/// Takes in the [`Abstract Syntax Tree`] from the DC parser and outputs a
/// [`crate::dcfile::DCFile`] immutable structure with a static lifetime.
///
/// [`Abstract Syntax Tree`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
pub fn semantic_analyzer<'a>(data: PipelineData) -> Result<dcfile::DCFile<'a>, ParseError> {
    let mut dc_file: dcfile::intermediate::DCFile = dcfile::intermediate::DCFile::default();

    // Iterate through all ASTs and add them to our DCFile intermediate object.
    for ast in data.syntax_trees {
        for type_declaration in ast.type_declarations {
            match type_declaration {
                ast::TypeDeclaration::PythonImport(import) => {
                    dc_file.add_python_import(import);
                }
                ast::TypeDeclaration::KeywordType(_) => {}
                ast::TypeDeclaration::StructType(_) => {}
                ast::TypeDeclaration::DClassType(_) => {}
                ast::TypeDeclaration::TypedefType(_) => {}
                // Ignore is returned by productions that parsed certain
                // grammar that may be deprecated but ignored for
                // compatibility & should not be added to the DC file.
                ast::TypeDeclaration::Ignore => {}
            }
        }
    }
    // Convert intermediate DC file structure to final immutable DC file structure.
    Ok(dc_file.into())
}

#[cfg(test)]
mod unit_testing {
    use super::*;
    use crate::read_dc;
    use dcfile::DCPythonImport;

    #[test]
    fn python_imports() {
        let dc_string: &str = "
            from views import *
            from views import DistributedDonut
            from views import Class/AI/OV
        ";

        let dcf: dcfile::DCFile = read_dc(dc_string.into()).expect("Failed to parse syntax.");

        let num_imports: usize = dcf.get_num_imports();
        assert_eq!(num_imports, 3);

        let symbols: Vec<Vec<String>> = vec![
            vec!["*".into()],
            vec!["DistributedDonut".into()],
            vec!["Class".into(), "ClassAI".into(), "ClassOV".into()],
        ];

        for index in 0..num_imports - 1 {
            let import: &DCPythonImport = dcf.get_python_import(index);

            assert_eq!(import.module, "views");

            let target_symbols: &Vec<String> = symbols.get(index).unwrap();

            assert_eq!(*target_symbols, import.symbols);
        }
    }
}
