use std::error::Error;
use std::fs::read_to_string;
use std::cmp;

pub fn compare_files_with_header( left : & str, right : & str )
        -> Result< (), Box< dyn Error > > {
    let left = read_to_string( left )?;
    let right = read_to_string( right )?;
    let left_lines : Vec< _ > = left.split( "\n" ).collect();
    let right_lines : Vec< _ > = right.split( "\n" ).collect();
    // Skip line 2 containing the generation date and time
    let compare_line_count = cmp::min( left_lines.len(), right_lines.len() );
    for i in 3..compare_line_count {
        assert_eq!( left_lines[ i ],
                right_lines[ i ], " line {}", i + 1 );
    }
    assert_eq!( left_lines.len(), right_lines.len() );
    Ok(())
}

