use ratatui::{layout::Layout, prelude::*, widgets::*};
use tui_textarea::{TextArea, Input, Key};

pub fn ui(frame: &mut Frame){
// frame.render_widget(
//         Paragraph::new("Hello World!")
//             .block(Block::default().title("Greeting").borders(Borders::ALL)),
//         frame.size(),
//     );
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    
    //Title Bar
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        main_layout[0],
    );

    //Status Bar
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[2],
    );



    //Middle Layout
    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(70), Constraint::Percentage(30)],
    )
    .split(main_layout[1]);
    
    //Left Part
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Left"),
        inner_layout[0],
    );
    
    let inner_left_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(100)],
        )
        .split(inner_layout[0]);


    frame.render_widget(
        TextArea::default()
            .widget(),
        inner_left_layout[0]);


    //Right Part
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Right"),
        inner_layout[1],
    );
    

    // input_printer(frame, inner_left_layout[0]);
}


pub fn input_printer(frame: &mut Frame, inner_layout:Rect){
    loop {
        let mut textarea = TextArea::default();
        // Input 'a'
        let input = Input { key: Key::Char('a'), ctrl: true, alt: true, shift: true };
        textarea.input(input);

        // Get widget to render.
        let _widget = textarea.widget();
    }   
}
