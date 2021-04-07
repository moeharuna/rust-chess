use raylib::prelude::*;
use raylib::ffi::LoadImageFromMemory;
use std::ffi::CString;
use std::collections::HashMap;

use super::pieces::{*, PieceType::*, PieceColor::*};
use super::board::Board;


struct RayLibState
{
    handle:RaylibHandle,
    thread:RaylibThread,
}

impl RayLibState
{
    pub fn start_drawing(&mut self) -> RaylibDrawHandle
    {
        self.handle.begin_drawing(&self.thread)
    }
    pub fn load_png_image(&mut self, texture_str:&[u8],) -> Texture2D
    {
        let png_str = CString::new("png").expect("Woopsie Doopse. Somehow creating CString from  string literal failed.").into_raw();
        let image:Image;
        unsafe
        {
            image = Image::from_raw(LoadImageFromMemory(png_str, texture_str.as_ptr(), texture_str.len() as i32));
            let png_str = CString::from_raw(png_str); // i do this to let rust manage memory properly
            drop(png_str);
        }
        self.handle.load_texture_from_image(&self.thread, &image).expect("Loading image from texture failed")
    }
    pub fn should_close(&self) -> bool
    {
        self.handle.window_should_close()
    }
}


//TODO: Decouple struct as raylib_state, raylibOnly_function, drawing_function_that_require_board, input(?)
pub struct PixelBoard
{
    raylib:RayLibState,
    board:Board,
    selected_piece:Option<Piece>,
    textures:HashMap<PieceType, Texture2D>,
    height:i32,
    width:i32,
}

impl PixelBoard
{
    fn init_textures(raylib:&mut RayLibState) -> HashMap<PieceType, Texture2D>
    {
        let mut textures = HashMap::new();
        textures.insert(Pawn(Black),   raylib.load_png_image(&include_bytes!("../res/black_pawn.png")[..])); //FIXME: Its probably good idea to use here match somehow and force rust to check all variants
        textures.insert(Pawn(White),   raylib.load_png_image(&include_bytes!("../res/white_pawn.png")[..]));
        textures.insert(Rook(Black),   raylib.load_png_image(&include_bytes!("../res/black_rook.png")[..]));
        textures.insert(Rook(White),   raylib.load_png_image(&include_bytes!("../res/white_rook.png")[..]));
        textures.insert(Knight(Black), raylib.load_png_image(&include_bytes!("../res/black_knight.png")[..]));
        textures.insert(Knight(White), raylib.load_png_image(&include_bytes!("../res/white_knight.png")[..]));
        textures.insert(Bishop(Black), raylib.load_png_image(&include_bytes!("../res/black_bishop.png")[..]));
        textures.insert(Bishop(White), raylib.load_png_image(&include_bytes!("../res/white_bishop.png")[..]));
        textures.insert(Queen(Black),  raylib.load_png_image(&include_bytes!("../res/black_queen.png")[..]));
        textures.insert(Queen(White),  raylib.load_png_image(&include_bytes!("../res/white_queen.png")[..]));
        textures.insert(King(Black),   raylib.load_png_image(&include_bytes!("../res/black_king.png")[..]));
        textures.insert(King(White),   raylib.load_png_image(&include_bytes!("../res/white_king.png")[..]));

        for (_, i) in textures.iter_mut()
        {
            i.gen_texture_mipmaps();
            i.set_texture_filter(&raylib.thread, raylib::ffi::TextureFilterMode::FILTER_ANISOTROPIC_16X);
        }
        textures
    }
    pub fn new(board:Board, width:i32, height:i32, title:&str) ->  PixelBoard
    {
        let (handle, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();
        let mut state = RayLibState{handle, thread};
        let textures = Self::init_textures(&mut state);
        PixelBoard{raylib:state, board, selected_piece:None, textures, width, height}
    }


    fn pixel2cell(&self, pixel: Vector2) -> Point
    {
        let (cell_width, cell_height) = self.get_cell_wh();
        let cell_width = cell_width as f32;
        let cell_height = cell_height  as f32;
        Point{x:(pixel.x/cell_width).ceil() as i32,
              y:(pixel.y/cell_height).ceil() as i32}


}
    fn cell2pixel(&self, cell: Point) -> Rectangle
    {
        let (pixel_width, pixel_height)  = self.get_cell_wh();
        let x:f32 = (cell.x*pixel_width) as f32;
        let y:f32 = (cell.y*pixel_height) as f32;
        Rectangle::new(x,y, pixel_width as f32, pixel_height as f32)

    }
    ///Function that returns postion of mouse where x and y are board squares, not pixels on screen.
    ///Be aware that this function still can return negative values and values largers that Board.board_size()
    pub fn get_mouse_pos(&self) -> Point
    {
        let pixel_mouse_pos = self.raylib.handle.get_mouse_position();
        let mut cell_mouse_pos = self.pixel2cell(pixel_mouse_pos);
        cell_mouse_pos.x = cell_mouse_pos.x-2;
        cell_mouse_pos.y = cell_mouse_pos.y-2;
        cell_mouse_pos
    }

/*
    pub fn tick(&mut self, board:&Board)
    {
        if self.raylib.handle.is_mouse_button_pressed(consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            let pos = self.get_mouse_pos(board);
//            self.selected_piece = *(board.get_piece_on_cell(pos).unwrap().clone()); //FIXME: selected piece should be reference or better it's should not exist
            //todo!("draw possible positions")
        }
        else if self.raylib.handle.is_mouse_button_released(consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            match self.selected_piece
            {
                None => (),
                Some(piece) => piece.set_position(self.get_mouse_pos(board))
            }
        }
    }
     */
    pub fn should_close(&self) -> bool
    {
        self.raylib.should_close()
    }

    fn get_cell_count_wh(b:&Board) -> (i32, i32)
    {
        let (width, height) = b.board_size();
        (1 + width as i32,
         1 + height as i32)
    }

    fn get_cell_wh(&self) -> (i32, i32)
    {
        let (cell_width, cell_height) = Self::get_cell_count_wh(&self.board);
        let size = std::cmp::min(self.width, self.height);
        ( size/cell_width,
          size/cell_height)
    }
    //TODO: decouple this function
    pub fn draw_board(
        &mut self,
        x:i32, y:i32)
    {


        let (rect_width, rect_height) = self.get_cell_wh();
        let (board_width, board_height) = self.board.board_size();
        let height_offset = y as i32 + rect_height+1;
        let width_offset  = x as i32 + rect_width+1;
        let mut draw = self.raylib.handle.begin_drawing(&self.raylib.thread);
        draw.clear_background(Color::LIGHTGRAY);
        for i in 0..board_width as u8
        {
            draw.draw_text(&(('A' as u8 +i) as char).to_string(), (i as i32)*rect_width+width_offset, height_offset-rect_width, rect_width, Color::BLACK);
        }
        for i in 0..board_height as i32
        {
            draw.draw_text(&(i+1).to_string(), width_offset-rect_height, i*rect_height+height_offset, rect_height, Color::BLACK);
            for j in 0..board_width as i32
            {
                let color = if (i+j)%2==0 {Color::GRAY} else {Color::BROWN};
                draw.draw_rectangle( j*rect_width+width_offset, i*rect_height+height_offset,
                                     rect_width, rect_height, color);
            }
        }
        for i in self.board.pieces.iter()
        {
            let piece_pos_x =width_offset +  i.position.x as i32*rect_width;
            let piece_pos_y =height_offset +  i.position.y as i32*rect_height;
            let texture  = match self.textures.get(&i.piece_type)
            {
                Some(val) => val,
                None => panic!( format!("Somehow texture of this type ({:?}) not exist", i.piece_type))
            };
            draw_texture_with_scale(&mut draw, &texture, piece_pos_x, piece_pos_y, rect_width as f32, rect_height as f32)
        }
    }






}
fn draw_texture_with_scale(draw:&mut RaylibDrawHandle, texture:&Texture2D,  x:i32, y:i32, width:f32, height:f32,)
{
    let x = x as f32;
    let y = y as f32;
    let source_rec= Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32);
    let dest_rec = Rectangle::new(x, y, width, height);
    let origin = Vector2::new(0.0, 0.0);
    draw.draw_texture_pro(texture, source_rec, dest_rec, origin, 0.0, Color::WHITE);
}