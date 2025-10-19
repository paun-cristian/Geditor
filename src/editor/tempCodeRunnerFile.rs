if line.len() as u16 > width {
                    let mut x = 0;
                    let mut current_column: u16 = width;
                    while line.len() as u16 > width {
                        Terminal::print(&line[x..width as usize])?;
                        Terminal::move_cursor(&Position { x: 0, y: current_row + 1 })?;
                        Terminal::clear_line()?;
                        x += width as usize;
                        current_column = (line.len() - x) as u16;
                    }
                    Terminal::print(&line[x..current_column as usize])?;