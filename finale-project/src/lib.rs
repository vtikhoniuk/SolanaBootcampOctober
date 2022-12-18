use anchor_lang::prelude::*;
use std::vec;
use std::string::String;

//use self::BoardPoint::*;

declare_id!("CwxJQ6QSANFUJ8nRsQu6yoBfDX2E5mRam5kWxkL6q6eY");

#[program]
mod TicTacToe {
    use super::*;

    // Creates an account for the game
    pub fn new_game(ctx: Context<NewGame>, playerTwo: Pubkey) -> Result<()> {
        ctx.accounts
            .game
            .new([ctx.accounts.player1.key(), playerTwo])
    }

    // Select point on the board in the game account
    pub fn select_point(ctx: Context<SelectPoint>, x: String, y: String) -> Result<()> {
        let game: &mut Account<Game> = &mut ctx.accounts.game;

        // Get player index, but also act as a check
        let indx: usize = game.get_player_index(ctx.accounts.player.key()).unwrap();

        game.SelectPoint(x, y, indx)
    }
}

#[derive(Accounts)]
pub struct SelectPoint<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct NewGame<'info> {
    #[account(init,  payer = player1, space = 64 +  Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[repr(u8)]
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BoardPoint {
    Point1x1 = 1,
    Point1x2 = 2,
    Point1x3 = 3,
    Point2x1 = 4,
    Point2x2 = 5,
    Point2x3 = 6,
    Point3x1 = 7,
    Point3x2 = 8,
    Point3x3 = 9,
}

impl BoardPoint {
    pub fn new(x: &String, y: &String) -> Result<Self> {
        match (x.as_str(), y.as_str()) {
            ("1","1") => Ok(BoardPoint::Point1x1),
            ("1","2") => Ok(BoardPoint::Point1x2),
            ("1","3") => Ok(BoardPoint::Point1x3),
            ("2","1") => Ok(BoardPoint::Point2x1),
            ("2","2") => Ok(BoardPoint::Point2x2),
            ("2","3") => Ok(BoardPoint::Point2x3),
            ("3","1") => Ok(BoardPoint::Point3x1),
            ("3","2") => Ok(BoardPoint::Point3x2),
            ("3","3") => Ok(BoardPoint::Point3x3),
            _ => Err(SErrors::WrongBoardPoint.into()),
        }
    }
}

impl Default for BoardPoint {
    fn default() -> Self {
        BoardPoint::Point1x1
    }
}

pub trait WinningCombo {
    fn combo(&self) -> Vec::<(BoardPoint,BoardPoint,BoardPoint)>;
}

impl WinningCombo for BoardPoint {
    fn combo(&self) -> Vec::<(BoardPoint,BoardPoint,BoardPoint)> {
		let mut points = Vec::<(BoardPoint,BoardPoint,BoardPoint)>::new();
		points.push((BoardPoint::Point1x1, BoardPoint::Point1x2, BoardPoint::Point1x3));
		points.push((BoardPoint::Point1x1, BoardPoint::Point2x1, BoardPoint::Point3x1));
		points.push((BoardPoint::Point3x1, BoardPoint::Point3x2, BoardPoint::Point3x3));
		points.push((BoardPoint::Point1x3, BoardPoint::Point2x3, BoardPoint::Point3x3));
		points.push((BoardPoint::Point1x1, BoardPoint::Point2x2, BoardPoint::Point3x3));
		points.push((BoardPoint::Point1x3, BoardPoint::Point2x2, BoardPoint::Point3x1));
		points
    }
}


// Account
////////////////////////////////////////////////////////////////

#[account]
pub struct Game {
    players: [Pubkey; 2],
    playerOneMoves: [BoardPoint; 5],
    playerOneMovePos: u8,
    playerTwoMoves: [BoardPoint; 5],
    playerTwoMovePos: u8,
    winner: String,
}

impl Game {
    // Based on account varfiable sizes
    pub const MAXIMUM_SIZE: usize = (32 * 2) + (32 * 2) + 3 * 2;

    // Player that pays for account set up calls this with both pubkeys
    fn new(&mut self, players: [Pubkey; 2]) -> Result<()> {
        self.players = players;
		self.playerOneMoves = [BoardPoint::Point1x1; 5];
        self.playerOneMovePos = 0;
		self.playerTwoMoves = [BoardPoint::Point1x1; 5];
        self.playerTwoMovePos = 0;
		self.winner = "DRAW".to_string();

        Ok(())
    }

    pub fn get_player_index(&mut self, player: Pubkey) -> Result<usize> {
        let index_player: usize = self.players.iter().position(|&x| x == player).unwrap();

        match index_player {
            0 => Ok(index_player),
            1 => Ok(index_player),
            _ => Err(SErrors::MissingPlayer.into()),
        }
    }
	
	fn IsElementPresent(&self, moves: &[BoardPoint], index: &u8, elem: &BoardPoint) -> bool {
		let mut present = false;
		for i in 0..=*index {
			if moves[i as usize] == *elem {
				present = true;
				break;
			}
		}
		present
	}

    pub fn SelectPoint(&mut self, x: String, y: String, indx: usize) -> Result<()> {
        // Extract the first word
        let board_point = BoardPoint::new(&x, &y).unwrap();
		
		if self.winner != "DRAW".to_string() {
			return Err(SErrors::GameOver.into());
		}
		
		if self.playerOneMovePos + self.playerTwoMovePos >= 9 {
			return Err(SErrors::GameOver.into());
		}
		
		if self.IsElementPresent(&self.playerOneMoves, &self.playerOneMovePos, &board_point) ||
		   self.IsElementPresent(&self.playerTwoMoves, &self.playerTwoMovePos, &board_point) {
			return Err(SErrors::WrongMove.into());
		}
		
		let playerMoves: &[BoardPoint; 5];
		let playerMovePos: &u8;
		if indx == 0 {
			self.playerOneMoves[self.playerOneMovePos as usize] = board_point;
			self.playerOneMovePos += 1;
			playerMoves = &self.playerOneMoves;
			playerMovePos = &self.playerOneMovePos;
		} else {
			self.playerTwoMoves[self.playerTwoMovePos as usize] = board_point;
			self.playerTwoMovePos += 1;
			playerMoves = &self.playerTwoMoves;
			playerMovePos = &self.playerTwoMovePos;
		}
		
        let board_point = BoardPoint::new(&x, &y).unwrap();
		
		if playerMovePos >= &3u8 {
			for combo in board_point.combo().iter(){
				let mut condition = true;
				for point in [combo.0, combo.1, combo.2] {
					condition = condition && self.IsElementPresent(playerMoves, playerMovePos, &point);
				}
				
				if condition{
					self.winner = self.players[indx].to_string();
					return Ok(());
				}
			}		
		}

		return Ok(());
    }
}

// Errors
////////////////////////////////////////////////////////////////

#[error_code]
pub enum SErrors {
    MissingPlayer,
    WrongBoardPoint,
    WrongMove,
    GameOver,
}