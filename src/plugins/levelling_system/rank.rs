///  Copyright 2020 - 2021 The HarTex Project Developers
///
///  Licensed under the Apache License, Version 2.0 (the "License");
///  you may not use this file except in compliance with the License.
///  You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
///  Unless required by applicable law or agreed to in writing, software
///  distributed under the License is distributed on an "AS IS" BASIS,
///  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///  See the License for the specific language governing permissions and
///  limitations under the License.

use std::{
    future::Future,
    fs,
    pin::Pin
};

use image::{
    ImageBuffer,
    Rgb,
    RgbImage
};

use imageproc::{
    drawing::{
        draw_text_mut,
        draw_filled_rect
    },
    rect::Rect
};

use rusttype::{
    Font,
    Scale
};

use twilight_cache_inmemory::InMemoryCache;

use twilight_mention::ParseMention;

use twilight_model::{
    id::UserId
};

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    CommandError,
    PrecommandCheckParameters
};

use crate::system::{
    twilight_http_client_extensions::GetUserExperience,
    SystemResult
};

crate struct RankCommand;

impl Command for RankCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("rank")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user = arguments.next();

        Box::pin(levelling_system_rank_command(ctx, user.map(|inner| inner.to_string())))
    }
}

async fn levelling_system_rank_command(ctx: CommandContext<'_>, user: Option<String>) -> SystemResult<()> {
    let user_id = if let Some(user_to_parse) = user {
        if let Ok(id) = UserId::parse(&user_to_parse) {
            id
        }
        else if let Ok(id) = user_to_parse.parse() {
            UserId(id)
        }
        else {
            ctx.http_client
                .clone()
                .create_message(ctx.message.channel_id)
                .content("<:red_x:705623424675872859> Cannot find user.")?
                .allowed_mentions()
                .replied_user(false)
                .build()
                .reply(ctx.message.id)
                .await?;

            return Err(box CommandError("User not found".to_string()))
        }
    }
    else {
        ctx.author.id
    };

    let mut image: RgbImage = ImageBuffer::new(934, 282);
    
    // Sets each pixel to RGB Colour 60, 61, 64.
    image.pixels_mut().for_each(|pixel| {
        *pixel = Rgb([60u8, 61u8, 64u8])
    });
    
    let regular_vector = Vec::from(include_bytes!("../../../fonts/Montserrat-Regular.ttf") as &[u8]);
    let bold_vector = Vec::from(include_bytes!("../../../fonts/Montserrat-Bold.ttf") as &[u8]);
    let montserrat_regular = Font::try_from_vec(regular_vector).unwrap();
    let montserrat_bold = Font::try_from_vec(bold_vector).unwrap();

    let level_text_height = 30.0;
    let level_scale = Scale {
        x: level_text_height,
        y: level_text_height
    };

    let level_int_scale = Scale {
        x: 55.0,
        y: 55.0
    };

    let (_, level, experience) =  ctx.http_client.clone().get_user_experience(ctx.message.guild_id.unwrap(), user_id).await?;

    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), 720, 35, level_scale, &montserrat_regular, "Level");
    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), 800, 16, level_int_scale, &montserrat_bold, &level.to_string());

    let total_experience_to_next_level = (5 * level).pow(2) + 50 * level + 100;
    let percentage = experience / total_experience_to_next_level;

    // Region: Progress Bar Drawing
    let progress_bar_max_length = 840u32;

    // Region: Progress Bar Drawing Preparation - First Rectangle
    let first_rectangle_width = progress_bar_max_length;
    let first_rectangle_height = 20u32;
    let first_rectangle = Rect::at(30, 225).of_size(first_rectangle_width, first_rectangle_height);

    // Region: Progress Bar Drawing Preparation - Second Rectangle
    let second_rectangle_width = progress_bar_max_length - 20u32;
    let second_rectangle_height = first_rectangle_height + 20u32;
    let second_rectangle = Rect::at(40, 215).of_size(second_rectangle_width, second_rectangle_height);

    // Region: Progress Bar Drawing - The Overlapping Rectangles
    draw_filled_rect_mut(&mut image, first_rectangle, Rgb([163u8, 160u8, 152u8]));
    draw_filled_rect_mut(&mut image, second_rectangle, Rgb([163u8, 160u8, 152u8]));
    
    // Region: Progress Bar Background Drawing - The Four Circles for Rounded Corners
    let circle_radii = 10;

    let circle_one_centre = (30 + 10, 225);
    draw_filled_circle_mut(&mut image, circle_one_centre, circle_radii, Rgb([163u8, 160u8, 152u8]));  // First Circle

    let circle_two_centre = (30 + 10, 245);
    draw_filled_circle_mut(&mut image, circle_two_centre, circle_radii, Rgb([163u8, 160u8, 152u8]));

    image.save("rank_card/card.png")?;

    ctx.http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .allowed_mentions()
        .replied_user(false)
        .build()
        .reply(ctx.message.id)
        .attachment("rank_card.png", fs::read("rank_card/card.png")?)
        .await?;

    Ok(())
}
