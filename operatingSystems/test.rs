
static PIECES: [[char; 6]; 7] = [[' '; 6]; 7];

fn main()
{
    
    println!("Hi there");
    print_board();
}

fn print_board()
{
    for y in 0..7
    {
        for x in 0..6
        {
            print!("|{}", PIECES[y][x]);
        }
        print!("|\n");
    }
    print!("\n\n");
}

fn move(piece: char, spot: u8)
{
    for i in 7..0
    {
        if (PIECES[spot][i])
        {
            
        }
    }
}