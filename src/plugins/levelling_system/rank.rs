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
    pin::Pin,
    str::FromStr
};

use hyper::{
    Body,
    Client,
    Request,
    Uri,
};

use hyper_tls::HttpsConnector;

use image::{
    codecs::{
        gif::GifDecoder,
        png::PngDecoder
    },
    imageops::{
        overlay
    },
    DynamicImage,
    ImageBuffer,
    Rgb,
    RgbImage
};

use imageproc::{
    drawing::{
        draw_filled_rect_mut,
        draw_filled_circle_mut,
        draw_text_mut
    },
    rect::Rect
};

use num::FromPrimitive;

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
    CommandError
};

use crate::content_distribution_network::ContentDistributionNetwork;

use crate::std_extensions::{
    FormatAsIec8000013PrefixPostfixDecimalMultiplerString
};

use crate::system::{
    twilight_http_client_extensions::{
        GetGuildLeaderboard,
        GetUserExperience
    },
    SystemResult
};

use crate::utilities::{
    image_processing::pixel_is_in_circle
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

    let user = ctx.http_client.clone().user(user_id).await?;
    let (user_avatar, animated, user) = if let Some(user_found) = user {
        if let Some(avatar_hash) = user_found.clone().avatar {
            (avatar_hash.clone(), avatar_hash.starts_with("a_"), user_found.clone())
        }
        else {
            (String::new(), false, user_found)
        }
    }
    else {
        return Err(box CommandError("user not found.".to_string()));
    };

    let leaderboard = ctx.http_client.clone().get_guild_leaderboard(ctx.message.guild_id.unwrap()).await?;

    let https_connector = HttpsConnector::new();
    let hyper_client = Client::builder().build::<_, Body>(https_connector);
    let request = Request::builder()
        .method("GET")
        .uri(Uri::from_str(&{
            if !user_avatar.is_empty() {
                ContentDistributionNetwork::user_avatar(user.id, user_avatar, animated)?
            }
            else {
                ContentDistributionNetwork::default_user_avatar(user.discriminator.parse()?)?
            }
        })?)
        .body(Body::empty())?;
    let response = hyper_client.request(request).await?;
    let bytes = hyper::body::to_bytes(response).await?.to_vec();

    let mut dynamic_image = if animated {
        DynamicImage::from_decoder(GifDecoder::new(bytes.as_slice())?)
    }
    else {
        DynamicImage::from_decoder(PngDecoder::new(bytes.as_slice())?)
    }?.to_rgb8();

    let radius = ((&dynamic_image).width() / 2);
    dynamic_image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        if !pixel_is_in_circle(
            x - (radius),
            y - (radius),
            radius) {
            *pixel = Rgb([60u8, 61u8, 64u8])
        }
    });

    let mut image: RgbImage = ImageBuffer::new(934, 282);

    // Sets each pixel to RGB Colour 60, 61, 64.
    image.pixels_mut().for_each(|pixel| {
        *pixel = Rgb([60u8, 61u8, 64u8])
    });

    let regular_vector = Vec::from(include_bytes!("../../../fonts/Microsoft Yahei.ttf") as &[u8]);
    let bold_vector = Vec::from(include_bytes!("../../../fonts/Microsoft Yahei Bold.ttf") as &[u8]);
    let yahei_regular = Font::try_from_vec(regular_vector).unwrap();
    let yahei_bold = Font::try_from_vec(bold_vector).unwrap();

    let level_scale = Scale {
        x: 30.0,
        y: 30.0
    };

    let level_int_scale = Scale {
        x: 55.0,
        y: 55.0
    };

    let (_, level, experience) =  ctx.http_client.clone().get_user_experience(ctx.message.guild_id.unwrap(), user_id).await?;

    let level_text = level.to_string();
    let mut level_text_temp_width = 0.0;

    level_text.clone().chars().for_each(|character| {
        let glyph_width = yahei_bold.glyph(character).scaled(level_int_scale).h_metrics().advance_width;

        level_text_temp_width += glyph_width;
    });
    let level_int_text_width = u32::from_f32(level_text_temp_width).unwrap();
    let level_int_position = 860 - level_int_text_width;

    let mut level_string_text_temp_width = 0.0;
    "Level".chars().for_each(|character| {
        let glyph_width = yahei_regular.glyph(character).scaled(level_int_scale).h_metrics().advance_width;

        level_string_text_temp_width += glyph_width;
    });
    let level_string_width = u32::from_f32(level_string_text_temp_width).unwrap();

    draw_text_mut(&mut image, Rgb([66u8, 135u8, 245u8]), level_int_position, 16, level_int_scale, &yahei_bold, &level_text);
    draw_text_mut(&mut image, Rgb([66u8, 135u8, 245u8]), level_int_position - level_string_width - 5 + 35, 35, level_scale, &yahei_regular, "Level");

    let (rank_int, _) = leaderboard.iter().enumerate().find(|(index, entry)| {
        entry.user_id == user.id
    }).unwrap();
    let rank_int_text = format!("#{}", (rank_int + 1).to_string());
    let mut rank_int_text_temp_width = 0.0;

    rank_int_text.clone().chars().for_each(|character| {
        let glyph_width = yahei_bold.glyph(character).scaled(level_int_scale).h_metrics().advance_width;

        rank_int_text_temp_width += glyph_width;
    });

    let rank_int_text_width = u32::from_f32(rank_int_text_temp_width).unwrap();
    let rank_int_text_position = level_int_position - level_string_width - 5 - rank_int_text_width - 100;

    let mut rank_string_text_temp_width = 0.0;
    "Rank".chars().for_each(|character| {
        let glyph_width = yahei_regular.glyph(character).scaled(level_int_scale).h_metrics().advance_width;

        rank_string_text_temp_width += glyph_width;
    });
    let rank_string_width = u32::from_f32(rank_string_text_temp_width).unwrap();

    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), rank_int_text_position, 16, level_int_scale, &yahei_bold, &rank_int_text);
    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), rank_int_text_position - rank_string_width - 5 + 35, 35, level_scale, &yahei_regular, "Rank");

    let total_experience_to_next_level = (5 * level).pow(2) + 50 * level + 100;
    let percentage = f64::from_u64(experience).unwrap() / f64::from_u64(total_experience_to_next_level).unwrap();

    // Region: Progress Bar Background Drawing
    let progress_bar_max_length = 840u32;

    // Region: Progress Bar Background Drawing Preparation - First Rectangle
    let first_rectangle_width = progress_bar_max_length;
    let first_rectangle_height = 20u32;
    let first_rectangle = Rect::at(30, 225).of_size(first_rectangle_width, first_rectangle_height);

    // Region: Progress Bar Background Drawing Preparation - Second Rectangle
    let second_rectangle_width = progress_bar_max_length - 20u32;
    let second_rectangle_height = first_rectangle_height + 20u32;
    let second_rectangle = Rect::at(40, 215).of_size(second_rectangle_width, second_rectangle_height);

    // Region: Progress Bar Background Drawing - The Overlapping Rectangles
    draw_filled_rect_mut(&mut image, first_rectangle, Rgb([163u8, 160u8, 152u8]));
    draw_filled_rect_mut(&mut image, second_rectangle, Rgb([163u8, 160u8, 152u8]));

    // Region: Progress Bar Background Drawing - The Four Circles for Rounded Corners
    let circle_radii = 10;

    let circle_one_centre = (30 + 10, 225);
    draw_filled_circle_mut(&mut image, circle_one_centre, circle_radii, Rgb([163u8, 160u8, 152u8]));  // First Circle

    let circle_two_centre = (30 + 10, 244);
    draw_filled_circle_mut(&mut image, circle_two_centre, circle_radii, Rgb([163u8, 160u8, 152u8]));  // Second Circle

    let circle_three_centre = ((30 + 9 + second_rectangle_width) as i32, 225);
    draw_filled_circle_mut(&mut image, circle_three_centre, circle_radii, Rgb([163u8, 160u8, 152u8]));  // Third Circle

    let circle_four_centre = ((30 + 9 + second_rectangle_width) as i32, 244);
    draw_filled_circle_mut(&mut image, circle_four_centre, circle_radii, Rgb([163u8, 160u8, 152u8]));  // Fourth Circle

    // Region: Progress Bar Foreground Drawing - First Rectangle
    let foreground_first_rectangle_width = u32::from_f64((f64::from(progress_bar_max_length) * percentage).round()).unwrap();

    if foreground_first_rectangle_width != 0 {
        let foreground_first_rectangle = Rect::at(30, 225).of_size(foreground_first_rectangle_width, 20u32);
        draw_filled_rect_mut(&mut image, foreground_first_rectangle, Rgb([66u8, 135u8, 245u8]));
    
        // Region: Progress Bar Foreground Drawing - Second Rectangle
        let foreground_second_rectangle_width = if (0u32..=8u32).contains(&foreground_first_rectangle_width) {
            0
        }
        else {
            foreground_first_rectangle_width - 20u32
        };

        if foreground_second_rectangle_width != 0 {
            let foreground_second_rectangle = Rect::at(40, 215).of_size(foreground_second_rectangle_width, 40u32);
            draw_filled_rect_mut(&mut image, foreground_second_rectangle, Rgb([66u8, 135u8, 245u8]));

            // Progress Bar Foreground Drawing - The Four Circles for Rounded Corners
            draw_filled_circle_mut(&mut image, circle_one_centre, circle_radii, Rgb([66u8, 135u8, 245u8]));  // First Circle
            draw_filled_circle_mut(&mut image, circle_two_centre, circle_radii, Rgb([66u8, 135u8, 245u8]));  // Second Circle

            let circle_three_centre = (foreground_first_rectangle.right() - 10, 225);
            draw_filled_circle_mut(&mut image, circle_three_centre, circle_radii, Rgb([66u8, 135u8, 245u8]));  // Third Circle

            let circle_four_centre = (foreground_first_rectangle.right() - 10, 244);
            draw_filled_circle_mut(&mut image, circle_four_centre, circle_radii, Rgb([66u8, 135u8, 245u8]));  // Fourth Circle
        }
    }

    // Region: / {integer & IEC 80000-13 Decimal Multiplier Standard} XP

    let mut temp_width = 0.0;
    let text = format!(
        "/ {} XP",
        total_experience_to_next_level.format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string()
    );

    let mut xp_temp_width = 0.0;
    let xp_text = experience.format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string();

    text.clone().chars()
        .for_each(|character| {
            let glyph_width = yahei_regular.glyph(character).scaled(Scale { x: 30.0, y: 30.0 }).h_metrics().advance_width;
                
            temp_width += glyph_width;
        });

    xp_text.clone().chars()
        .for_each(|character| {
            let glyph_width = yahei_bold.glyph(character).scaled(Scale { x: 30.0, y: 30.0 }).h_metrics().advance_width;

            xp_temp_width += glyph_width;
        });

    let expected_text_width = u32::from_f32(temp_width).unwrap();
    let out_of_xp_text_position = 860 - expected_text_width;
    let xp_expected_text_width = u32::from_f32(xp_temp_width).unwrap() + 5;

    draw_text_mut(&mut image, Rgb([164u8, 176u8, 176u8]), out_of_xp_text_position, 170, Scale { x: 28.5, y: 28.5 }, &yahei_regular, &text);
    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), out_of_xp_text_position - xp_expected_text_width, 170, Scale { x: 28.5, y: 28.5 } , &yahei_bold, &experience.format_as_iec_80000_13_prefix_postfix_decimal_multiplier_string());

    // Region: Draw User Avatar on Rank Card

    overlay(&mut image, &dynamic_image, 50, 50);

    // Region: Text - User Username
    
    let base_position = 50 + radius * 2;

    let username_text = user.name;
    let mut temp_username_text_width = 0.0;

    username_text.clone().chars().for_each(|character| {
        let glpyh_width = yahei_regular.glyph(character).scaled(Scale { x: 50.0, y: 50.0 }).h_metrics().advance_width;

        temp_username_text_width += glpyh_width;
    });

    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), base_position + 10, 140, Scale { x: 50.0, y: 50.0 }, &yahei_regular, &username_text);

    // Region: Text - User Discriminator
    let expected_username_text_width = u32::from_f32(temp_username_text_width).unwrap();

    draw_text_mut(&mut image, Rgb([164u8, 176u8, 176u8]), base_position + expected_username_text_width + 10, 140, Scale { x: 50.0, y: 50.0 }, &yahei_regular, &format!("#{}", user.discriminator));

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
