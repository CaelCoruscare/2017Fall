#[macro_use] extern crate text_io;

fn main()
{
    let mut piece: [[char; 6]; 7] = [[' '; 6]; 7];

    println!("Hi there");
    print_board(&piece);
    make_move('o', 4, &mut piece);
    print_board(&piece);
    //eufsekfhesu
}

fn pvc_game()
{
    let m: i32;
    loop
    {
        printf!("Make your move.\n");
        i = read!();
        make_move('x', i, &mut piece);
        i = rand::random(1,6);
        make_move('o', i, &mut piece);
    }
}

fn print_board(pieces: &[[char; 6]; 7])
{
    for y in (0..7).rev()
    {
        for x in 0..6
        {
            print!("|{}", pieces[y][x]);
        }
        print!("|\n");
    }
    print!(" 1 2 3 4 5 6\n\n");
}

fn make_move(piece: char, spot: usize, pieces: &mut [[char; 6]; 7])
{
    //let i: usize;
    for i in 0..6
    {
        if pieces[i][spot-1] == ' '
        {
            pieces[i][spot-1] = piece;
            break;
        }
    }
}