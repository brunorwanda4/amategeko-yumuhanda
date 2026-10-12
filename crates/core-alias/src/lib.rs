#[cfg(test)]
mod tests {
    #[test]
    fn web_key_helper() {
        let key =
            amategeko_core::platform::web_storage_key(amategeko_core::platform::SETTINGS_FILE);
        assert_eq!(key, "amategeko:settings.json");
        assert_eq!(
            amategeko_core::platform::web_storage_key_to_filename(&key),
            Some(amategeko_core::platform::SETTINGS_FILE)
        );
    }
}

#[cfg(test)]
mod swipe_tests {
    use amategeko_core::{
        quiz_swipe_target, quiz_swipe_target_with_direction, swipe_decision, target_page,
        Direction, QuizMode,
    };

    #[test]
    fn test_swipe_decision_and_target_page() {
        let screen_w = 360.0;
        let edge = 24.0;

        // short drag is ignored
        assert_eq!(
            swipe_decision((100.0, 200.0), (140.0, 200.0), 200, screen_w, edge),
            None
        );

        // mostly vertical drag is ignored
        assert_eq!(
            swipe_decision((100.0, 200.0), (200.0, 260.0), 200, screen_w, edge),
            None
        );

        // long fast horizontal drag works in both directions
        assert_eq!(
            swipe_decision((200.0, 200.0), (100.0, 200.0), 200, screen_w, edge),
            Some(Direction::Left)
        );
        assert_eq!(
            swipe_decision((100.0, 200.0), (200.0, 200.0), 200, screen_w, edge),
            Some(Direction::Right)
        );

        // start inside edge zone is ignored
        assert_eq!(
            swipe_decision((10.0, 200.0), (200.0, 200.0), 200, screen_w, edge),
            None
        );
        assert_eq!(
            swipe_decision((350.0, 200.0), (150.0, 200.0), 200, screen_w, edge),
            None
        );

        // first page + swipe right and last page + swipe left return None
        let total_pages = 5;
        assert_eq!(target_page(0, Direction::Right, total_pages), None);
        assert_eq!(
            target_page(total_pages - 1, Direction::Left, total_pages),
            None
        );

        // valid navigation moves
        assert_eq!(target_page(0, Direction::Left, total_pages), Some(1));
        assert_eq!(target_page(1, Direction::Right, total_pages), Some(0));
    }

    #[test]
    fn swipe_rules_follow_mode_answers_boundaries_and_direction() {
        assert_eq!(
            quiz_swipe_target(QuizMode::Bikomeye, 1, 3, true, Direction::Left),
            None
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Byoroshye, 1, 3, false, Direction::Right),
            Some(0)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Byoroshye, 1, 3, false, Direction::Left),
            None
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Byoroshye, 1, 3, true, Direction::Left),
            Some(2)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 1, 3, false, Direction::Right),
            Some(0)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 1, 3, false, Direction::Left),
            Some(2)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Study, 1, 3, false, Direction::Right),
            Some(0)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Study, 1, 3, true, Direction::Left),
            Some(2)
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 0, 3, false, Direction::Right),
            None
        );
        assert_eq!(
            quiz_swipe_target(QuizMode::Hagati, 2, 3, false, Direction::Left),
            None
        );
        assert_eq!(
            quiz_swipe_target_with_direction(QuizMode::Hagati, 1, 3, false, Direction::Left, false),
            Some(0)
        );
    }
}
