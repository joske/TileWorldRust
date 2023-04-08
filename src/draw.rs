use crate::{
    grid::Grid,
    location::Location,
    objects::{AgentInfo, GO},
    COLS, MAG, ROWS,
};
use graphics::{
    color::{BLACK, BLUE, GREEN, RED},
    types::Color,
    CharacterCache, CircleArc, Context, Ellipse, Graphics, Rectangle, Text,
};
use std::f64::consts::PI;

pub fn draw<G: Graphics, C>(
    grid: &Grid,
    agents: &[AgentInfo],
    glyphs: &mut C,
    ctx: &Context,
    g: &mut G,
) where
    C: CharacterCache<Texture = G::Texture>,
{
    let width = f64::from(u16::saturating_mul(COLS, MAG as u16));
    let height = f64::from(u16::saturating_mul(ROWS, MAG as u16));
    Rectangle::new_border(BLACK, 1.0).draw(
        [0.0, 0.0, width, height],
        &ctx.draw_state,
        ctx.transform,
        g,
    );
    for r in 0..ROWS {
        for c in 0..COLS {
            let x = f64::from(u16::saturating_mul(c, MAG as u16));
            let y = f64::from(u16::saturating_mul(r, MAG as u16));
            let l = Location::new(c, r);
            if !grid.is_free(&l) {
                if let Some(go) = grid.object(&l) {
                    match *go.borrow() {
                        GO::Agent(ref a) => {
                            let color = get_color(a.id - 1);
                            Rectangle::new_border(color, 1.0).draw(
                                [x, y, MAG, MAG],
                                &ctx.draw_state,
                                ctx.transform,
                                g,
                            );
                            if a.has_tile {
                                let score = a.tile.as_ref().unwrap().borrow().score();
                                CircleArc::new(color, 1.0, 0.0, 2f64 * PI).draw(
                                    [x, y, MAG, MAG],
                                    &ctx.draw_state,
                                    ctx.transform,
                                    g,
                                );
                                Text::new_color(color, 14)
                                    .draw_pos(
                                        &score.to_string(),
                                        [x + MAG / 4f64, y + MAG - 4.0],
                                        glyphs,
                                        &ctx.draw_state,
                                        ctx.transform,
                                        g,
                                    )
                                    .unwrap();
                            }
                        }
                        GO::Hole(_) => {
                            Ellipse::new(BLACK).draw_from_to(
                                [x, y],
                                [x + MAG, y + MAG],
                                &ctx.draw_state,
                                ctx.transform,
                                g,
                            );
                        }
                        GO::Tile(ref t) => {
                            let score = t.score.to_string();
                            CircleArc::new(BLACK, 1.0, 0.0, 2f64 * PI).draw(
                                [x, y, MAG, MAG],
                                &ctx.draw_state,
                                ctx.transform,
                                g,
                            );
                            Text::new_color(BLACK, 14)
                                .draw_pos(
                                    score.as_str(),
                                    [x + MAG / 4f64, y + MAG - 4.0],
                                    glyphs,
                                    &ctx.draw_state,
                                    ctx.transform,
                                    g,
                                )
                                .unwrap();
                        }
                        GO::Obstacle(_) => {
                            Rectangle::new(BLACK).draw(
                                [x, y, MAG, MAG],
                                &ctx.draw_state,
                                ctx.transform,
                                g,
                            );
                        }
                    }
                }
            }
        }
    }
    let x = COLS as f64 * MAG + 50_f64;
    let y = 50f64;

    for agent in agents {
        let id = agent.id;
        let score = agent.score;
        let color = get_color(id - 1);
        let text = format!("Agent({id}): {score}");
        Text::new_color(color, 12)
            .draw_pos(
                text.as_str(),
                [x, y + id as f64 * MAG],
                glyphs,
                &ctx.draw_state,
                ctx.transform,
                g,
            )
            .unwrap();
    }
}

fn get_color(num: u8) -> Color {
    match num {
        0 => BLUE,
        1 => RED,
        2 => GREEN,
        3 => [0.5, 0.5, 0., 1.],
        4 => [0., 0.5, 0.5, 1.],
        5 => [0.5, 0., 0.5, 1.],
        _ => [0.5, 0.5, 0.5, 1.],
    }
}
