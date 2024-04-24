use crate::terminal::canvas::Canvas;
use crate::ui::color_scheme;
use luola::stat::Stat;
use luola::world::Layer;

pub struct CreatureInfo {
    width: usize,
    height: usize,
}

impl CreatureInfo {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn render(&self, creature_id: Option<u128>, layer: &Layer) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        if creature_id.is_none() {
            return canvas;
        }

        let creature_id = creature_id.unwrap();
        let creature = layer.creatures.get(&creature_id).unwrap();

        let bullet_point = "\u{25ba}";

        canvas.write(creature.name.clone(), color_scheme::TEXT_HIGHLIGHT_STYLE);
        canvas.write(
            format!(" (Level {})", creature.stats.level),
            color_scheme::TEXT_STYLE,
        );
        canvas.write_newline();
        canvas.write_newline();

        let current_hp = creature.stats.current_hp as f64;
        let max_hp = creature.stats.max_hp.get_value(creature.stats.level) as f64;

        let hp_percentage: f64 = if max_hp != 0.0 {
            (current_hp / max_hp) * 100.0
        } else {
            0.0
        };

        canvas.write(String::from("HP "), color_scheme::TEXT_HIGHLIGHT_STYLE);
        canvas.write(
            format!("{} ({:.1} %)", creature.stats.current_hp, hp_percentage),
            color_scheme::TEXT_STYLE,
        );
        canvas.write_newline();
        canvas.write(String::from("Max HP "), color_scheme::TEXT_HIGHLIGHT_STYLE);
        write_value_stat(&mut canvas, &creature.stats.max_hp, creature.stats.level);
        canvas.write_newline();
        canvas.write_newline();

        canvas.write(String::from("Attacks"), color_scheme::TEXT_HIGHLIGHT_STYLE);
        canvas.write_newline();
        canvas.write(
            format!("{} Melee ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_bonus_stat(
            &mut canvas,
            &creature.stats.melee_attack,
            creature.stats.level,
        );
        canvas.write_newline();
        canvas.write(
            format!("{} Ranged ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_bonus_stat(
            &mut canvas,
            &creature.stats.ranged_attack,
            creature.stats.level,
        );
        canvas.write_newline();
        canvas.write(
            format!("{} Magic ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_bonus_stat(
            &mut canvas,
            &creature.stats.magic_attack,
            creature.stats.level,
        );
        canvas.write_newline();
        canvas.write_newline();

        canvas.write(String::from("Defenses"), color_scheme::TEXT_HIGHLIGHT_STYLE);
        canvas.write_newline();
        canvas.write(
            format!("{} Armor ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_value_stat(
            &mut canvas,
            &creature.stats.armor_class,
            creature.stats.level,
        );
        canvas.write_newline();
        canvas.write(
            format!("{} Fortitude ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_value_stat(
            &mut canvas,
            &creature.stats.fortitude_dc,
            creature.stats.level,
        );
        canvas.write_newline();
        canvas.write(
            format!("{} Reflex ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_value_stat(&mut canvas, &creature.stats.reflex_dc, creature.stats.level);
        canvas.write_newline();
        canvas.write(
            format!("{} Will ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_value_stat(&mut canvas, &creature.stats.will_dc, creature.stats.level);
        canvas.write_newline();
        canvas.write_newline();

        canvas.write(String::from("Other"), color_scheme::TEXT_HIGHLIGHT_STYLE);
        canvas.write_newline();
        canvas.write(
            format!("{} Speed ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_value_stat(
            &mut canvas,
            &creature.stats.movement_speed,
            creature.stats.level,
        );
        canvas.write_newline();
        canvas.write(
            format!("{} Initiative ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_bonus_stat(
            &mut canvas,
            &creature.stats.initiative,
            creature.stats.level,
        );
        canvas.write_newline();
        canvas.write(
            format!("{} Actions ", bullet_point),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        write_value_stat(&mut canvas, &creature.stats.n_actions, creature.stats.level);

        canvas
    }
}

fn write_bonus_stat(canvas: &mut Canvas, stat: &Stat, level: i32) {
    canvas.write(
        format!("{:+}", stat.get_value(level)),
        color_scheme::TEXT_STYLE,
    );
    write_stat_modifiers(canvas, stat);
}

fn write_value_stat(canvas: &mut Canvas, stat: &Stat, level: i32) {
    canvas.write(
        format!("{}", stat.get_value(level)),
        color_scheme::TEXT_STYLE,
    );
    write_stat_modifiers(canvas, stat);
}

fn write_stat_modifiers(canvas: &mut Canvas, stat: &Stat) {
    let add_modifier = stat.get_total_additive_modifier();
    let mul_modifier = stat.get_total_multiplicative_modifier();

    if stat.is_modified() {
        canvas.write(String::from(" ("), color_scheme::TEXT_STYLE);

        write_add_modifier(canvas, add_modifier);

        if add_modifier != 0 && mul_modifier != 1.0 {
            canvas.write(String::from(", "), color_scheme::TEXT_STYLE);
        }

        write_mul_modifier(canvas, mul_modifier);

        canvas.write(String::from(")"), color_scheme::TEXT_STYLE);
    }
}

fn write_add_modifier(canvas: &mut Canvas, add_modifier: i32) {
    if add_modifier == 0 {
        return;
    }

    let style = if add_modifier > 0 {
        color_scheme::GOOD_MODIFIER_STYLE
    } else {
        color_scheme::BAD_MODIFIER_STYLE
    };

    canvas.write(format!("{:+}", add_modifier), style);
}

fn write_mul_modifier(canvas: &mut Canvas, mul_modifier: f64) {
    if mul_modifier == 1.0 {
        return;
    }

    let style = if mul_modifier >= 1.0 {
        color_scheme::GOOD_MODIFIER_STYLE
    } else {
        color_scheme::BAD_MODIFIER_STYLE
    };

    // Multiplication sign (x)
    canvas.write(format!("\u{00d7}{:.2}", mul_modifier), style);
}
