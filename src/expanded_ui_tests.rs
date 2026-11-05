use super::*;
use crate::app::{EXPANDED_SCALE, HOST_H, HOST_W};

fn enlarged() -> H {
    let mut h=H::new();
    h.app.opts.expanded_scale=EXPANDED_SCALE;
    h
}

fn point(x:f32,y:f32)->Pos2 {
    Pos2::new(HOST_W/2.0+(x-PW/2.0)*EXPANDED_SCALE,y*EXPANDED_SCALE)
}

#[test]
fn enlarged_panel_keeps_collapsed_geometry_and_fonts_unchanged(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());
    assert_eq!(Options::default().expanded_scale,EXPANDED_SCALE);
    let mut original=H::new();let mut bigger=enlarged();
    for _ in 0..90{original.frame(vec![]);bigger.frame(vec![]);}
    assert_eq!(original.hit.size(),bigger.hit.size());
    assert_eq!(original.region.size(),bigger.region.size());
    assert_eq!(original.app.current_width(),bigger.app.current_width());
    let fonts=|h:&H|h.shapes.iter().filter_map(|shape|match &shape.shape {
        egui::Shape::Text(t)=>Some(t.galley.job.sections.iter().map(|s|s.format.font_id.size).collect::<Vec<_>>()),_=>None,
    }).collect::<Vec<_>>();
    assert_eq!(fonts(&original),fonts(&bigger));
}

#[test]
fn enlarged_panel_scales_five_pages_and_hit_regions_at_multiple_dpi(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());
    for dpi in [1.0,1.25,1.5,2.0] {
        let mut h=enlarged();h.ctx.set_pixels_per_point(dpi);
        for page in [0,3,4,5,7] {
            h.app.show_page(page);h.hover(Pos2::new(HOST_W/2.0,15.0),1.5);
            assert!(h.app.open);
            assert!((h.app.current_width()-PW*EXPANDED_SCALE).abs()<0.1);
            assert!((h.hit.height()-(h.app.expanded_height()*EXPANDED_SCALE+2.0)).abs()<0.1);
            assert!((h.hit.center().x-HOST_W/2.0).abs()<0.1);
            let settings=h.ctx.read_response(egui::Id::new(("card",true,"page-settings"))).unwrap().rect;
            assert!((settings.width()-28.0*EXPANDED_SCALE).abs()<0.1);
            assert!(h.hit.contains_rect(settings) && h.region.contains_rect(settings));
            assert!(h.region.bottom()<=HOST_H && h.region.left()>=0.0);
        }
    }
}

#[test]
fn enlarged_panel_keeps_text_size_when_switching_to_taller_calendar(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());let mut h=enlarged();
    h.hover(Pos2::new(HOST_W/2.0,15.0),1.5);
    h.app.show_page(7);h.frame(vec![]);
    let settings=h.ctx.read_response(egui::Id::new(("card",true,"page-settings"))).unwrap().rect;
    assert!((settings.width()-28.0*EXPANDED_SCALE).abs()<0.1,"切换页面高度时字号与按钮不能先缩小再放大");
}

#[test]
fn enlarged_panel_accepts_hover_in_added_area_and_returns_to_original_size(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());let mut h=enlarged();
    for _ in 0..90{h.frame(vec![]);}let collapsed=h.hit.size();let width=h.app.current_width();
    h.hover(Pos2::new(HOST_W/2.0,15.0),1.5);
    let added_area=point(PW-12.0,PH-12.0);
    assert!((added_area.x-HOST_W/2.0).abs()>PW/2.0 && added_area.y>PH);
    h.hover(added_area,2.0);assert!(h.app.open,"放大后新增区域应保持展开");
    let outside=Pos2::new(HOST_W+100.0,HOST_H);
    h.hover(outside,0.05);assert!(h.app.open,"保留移开延迟");
    h.hover(outside,1.5);assert!(!h.app.open);
    assert!((h.app.current_width()-width).abs()<0.1);
    assert!((h.hit.size()-collapsed).length()<0.1);
    for _ in 0..3 {
        h.hover(Pos2::new(HOST_W/2.0,15.0),0.3);
        let before=h.app.current_width();h.hover(outside,0.15);
        assert!((h.app.current_width()-before).abs()<PW,"中途反转不得跳变");
        h.hover(Pos2::new(HOST_W/2.0,15.0),0.8);assert!(h.app.open);
    }
}

#[test]
fn enlarged_panel_todo_input_and_editor_are_scaled_and_accept_text(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());let mut h=enlarged();
    h.app.show_page(4);h.hover(Pos2::new(HOST_W/2.0,15.0),1.5);
    h.app.adding=true;h.frame(vec![]);
    let input=h.ctx.read_response(egui::Id::new("todo-input")).unwrap().rect;
    assert!((input.width()-142.0*EXPANDED_SCALE).abs()<1.0);
    assert!(h.hit.contains_rect(input));
    h.frame(vec![egui::Event::Text("放大界面测试待办".into())]);
    h.frame(vec![egui::Event::Key{key:egui::Key::Enter,physical_key:None,pressed:true,repeat:false,modifiers:Default::default()}]);
    assert_eq!(h.app.todos.items[0].text,"放大界面测试待办");
    h.app.open_task_dialog(0);h.frame(vec![]);
    let editor=h.ctx.read_response(egui::Id::new("task-name-edit")).unwrap().rect;
    assert!((editor.width()-260.0*EXPANDED_SCALE).abs()<1.0);
    assert!(h.hit.contains_rect(editor));
}

#[test]
fn enlarged_panel_calendar_editor_and_day_selection_work_at_scaled_coordinates(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());let mut h=enlarged();
    h.app.open_calendar(None);h.hover(Pos2::new(HOST_W/2.0,15.0),1.5);
    h.app.open_calendar_editor(None);h.frame(vec![]);
    let input=h.ctx.read_response(egui::Id::new("calendar-local-title")).unwrap().rect;
    assert!((input.width()-272.0*EXPANDED_SCALE).abs()<1.0);
    assert!(h.hit.contains_rect(input));
    h.frame(vec![egui::Event::Text("放大日程输入".into())]);
    assert_eq!(h.app.calendar.draft.as_ref().unwrap().title,"放大日程输入");
    h.click(point(256.0,199.0));assert!(h.app.calendar.draft.is_none());
    h.frame(vec![]);
    let date=h.app.calendar.view.day+chrono::Duration::days(1);
    let day=h.ctx.read_response(egui::Id::new(("card",true,("calendar-day",date)))).unwrap().rect;
    h.click(day.center());assert_eq!(h.app.calendar.view.day,date);
}

#[test]
fn enlarged_panel_music_seek_uses_the_enlarged_track_position(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());let mut h=enlarged();
    h.media=crate::smtc::NowPlaying{source:"spotify".into(),title:"测试音乐".into(),artist:"测试歌手".into(),duration:200.0,position:10.0,playing:true,..Default::default()};
    h.hover(Pos2::new(HOST_W/2.0,15.0),1.5);
    let track=h.ctx.read_response(egui::Id::new(("card-drag",true,"m-seek"))).unwrap().rect;
    h.cmds.clear();h.click(track.center());
    assert!(h.cmds.iter().any(|cmd|matches!(cmd,crate::app::Cmd::Media(crate::smtc::Transport::Seek(seconds)) if (*seconds-100.0).abs()<2.0)));
}

#[cfg(feature="record")]
#[test]
fn enlarged_panel_mail_body_scroll_does_not_switch_pages(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());let mut h=enlarged();
    h.app.mail_demo();
    let account=h.app.mail.recent[0].account.clone();let uid=h.app.mail.recent[0].uid;
    h.app.mail.recent[0].summary="放大后的邮件正文应可滚动阅读。".repeat(40).into();
    h.app.mail.open_uid=Some((account.clone(),uid));h.app.show_page(5);
    h.hover(point(270.0,150.0),1.5);
    assert!((h.hit.height()-(236.0*EXPANDED_SCALE+2.0)).abs()<0.1);
    h.wheel(-120.0);
    assert_eq!(h.app.page(),5);
    assert_eq!(h.app.mail.open_uid,Some((account,uid)));
}

#[test]
fn enlarged_panel_shelf_drop_and_remove_keep_the_original_file(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());let mut h=enlarged();
    let file=crate::model::config_dir().join("drop-fixture.txt");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();std::fs::write(&file,"synthetic file").unwrap();
    h.frame(vec![]);let zone=h.file_drop_zone.unwrap();
    h.cursor=Some(zone.center());h.hovered_files.push(egui::HoveredFile{path:Some(file.clone()),..Default::default()});
    for _ in 0..90{h.frame(vec![]);}
    assert!(h.app.open && h.app.page()==3);
    assert!((h.file_drop_zone.unwrap().width()-PW*EXPANDED_SCALE).abs()<0.1);
    h.hovered_files.clear();h.dropped_files.push(egui::DroppedFile{path:Some(file.clone()),..Default::default()});
    h.frame(vec![]);h.hover(point(PW/2.0,60.0),1.5);
    assert!(h.app.shelf.items.contains(&file));
    let remove=h.ctx.read_response(egui::Id::new(("card",true,("shelf-remove",&file)))).unwrap().rect;
    h.click(remove.center());assert!(!h.app.shelf.items.contains(&file));assert!(file.is_file());
}

#[cfg(feature="record")]
#[test]
#[ignore="导出合成界面截图，不连接私人账号"]
fn enlarged_panel_preview_gallery(){
    let _g=ENV_LOCK.lock().unwrap_or_else(|e|e.into_inner());
    let directory=std::path::PathBuf::from(std::env::var_os("DAOZAI_PREVIEW_DIR").expect("设置截图输出目录"));
    std::fs::create_dir_all(&directory).unwrap();
    for scale in [1.0,EXPANDED_SCALE] {for page in [0,3,4,5,7] {
        let mut h=H::new();h.app.opts.expanded_scale=scale;h.app.mail_demo();h.app.calendar_demo();
        h.app.show_page(page);h.cursor=Some(Pos2::new(WIN_W*scale/2.0,15.0));
        let size=Vec2::new(crate::app::CANVAS_W,WIN_H*scale);
        let mut raster=crate::preview_raster::Raster::new(size.x as usize,size.y as usize);
        for frame in 0..90 {
            h.t+=1.0/60.0;
            let raw=egui::RawInput{screen_rect:Some(Rect::from_min_size(Pos2::ZERO,size)),time:Some(h.t),..Default::default()};
            let env=Env{t:h.t,clock:"14:15:00".into(),today:"2026-11-05".into(),date:"11月5日 周四".into(),cursor:h.cursor,bt_ready:true,..Default::default()};
            let output=h.ctx.run(raw,|ctx|{egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx,|ui|{h.app.frame(ui,Rect::from_min_size(Pos2::ZERO,Vec2::new(WIN_W,WIN_H)*scale),&env);});});
            raster.update_textures(&output.textures_delta);
            if frame==89 {
                raster.buf.fill([0.07,0.09,0.13,1.0]);
                raster.draw(&h.ctx.tessellate(output.shapes,output.pixels_per_point),output.pixels_per_point);
                let mut rgba=Vec::new();raster.to_rgba8(&mut rgba);
                image::save_buffer(directory.join(format!("page-{page}-{}percent.png",(scale*100.0)as u32)),&rgba,size.x as u32,size.y as u32,image::ColorType::Rgba8).unwrap();
            }
            raster.free_textures(&output.textures_delta);
        }
    }}
}
