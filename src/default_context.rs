use nu_protocol::engine::{EngineState, StateWorkingSet};

use std::path::Path;

use crate::*;

pub fn create_default_context(cwd: impl AsRef<Path>) -> EngineState {
    let mut engine_state = EngineState::new();

    let delta = {
        let mut working_set = StateWorkingSet::new(&engine_state);

        macro_rules! bind_command {
            ( $( $command:expr ),* $(,)? ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

        // If there are commands that have the same name as default declarations,
        // they have to be registered before the main declarations. This helps to make
        // them only accessible if the correct input value category is used with the
        // declaration

        // Core
        bind_command! {
            Alias,
            Debug,
            Def,
            DefEnv,
            Describe,
            Do,
            Du,
            Echo,
            ErrorMake,
            ExportAlias,
            ExportCommand,
            ExportDef,
            ExportDefEnv,
            ExportEnv,
            ExportExtern,
            Extern,
            For,
            Help,
            Hide,
            History,
            If,
            Ignore,
            Let,
            Metadata,
            Module,
            Source,
            Tutor,
            Use,
            Version,
        };

        // FileSystem
        bind_command! {
            Ls,
        };

        // Date
        bind_command! {
            Date,
            DateFormat,
            DateHumanize,
            DateListTimezones,
            DateNow,
            DateToRecord,
            DateToTable,
            DateToTimezone,
        };

        // Random
        bind_command! {
            Random,
            RandomBool,
            RandomChars,
            RandomDecimal,
            RandomDice,
            RandomInteger,
            //RandomUuid,
        };

        // Shells
        bind_command! {
            Exit,
        };

        // System
        bind_command! {
            External,
        };

        // Viewers
        bind_command! {
            //Griddle,
            Table,
        };

        working_set.render()
    };

    let _ = engine_state.merge_delta(delta, None, &cwd);

    engine_state
}
