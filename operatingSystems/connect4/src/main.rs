#[macro_use] extern crate text_io;
extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn main()
{
    let mut pieces: [[char; 6]; 7] = [[' '; 6]; 7];

    println!("Hi there");
    pvc_game(&mut pieces);
    //testing
}

fn pvc_game(mut pieces: &mut[[char; 6]; 7])
{
    let mut m: usize;
    let mut rng = rand::thread_rng();
    let gen = Range::new(1, 7);

    print_board(&pieces);

    loop
    {
        print!("Make your move.\n");
        m = read!();
        make_move('x', m, &mut pieces);
        print_board(&pieces);
        m = gen.ind_sample(&mut rng);
        make_move('o', m, &mut pieces);
        print_board(&pieces);
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
    print!("spot: {}\n", spot);
    //let i: usize;
    for i in 0..7
    {
        println!("pieces[i][spot-1]: {}", pieces[i][spot-1]);
        if pieces[i][spot-1] == ' '
        {
            pieces[i][spot-1] = piece;
            break;
        }
    }

    fn ai_move(pieces: [[char; 6]; 7])
    {
        
    }
}