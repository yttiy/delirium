
pub enum Message{
    CreateEntity{ e_type: i32, x: f32, y: f32 },
    CastSpell{ x: f32, y: f32, dx: f32, dy: f32, is_player_cast: bool, spell_type: i32 },
    Collision{ e1:i32, e2: i32 },
    PlaySound{ s_id: i32 },
    SendPlayerId{ id: i32 }
}