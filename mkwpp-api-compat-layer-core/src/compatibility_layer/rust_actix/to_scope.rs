use super::{from_input::InputFromActix, to_route::ToActixRoute};
use crate::{
    endpoint::{
        Endpoint, Root, Scope,
        blog::{BlogScope, GetBlogList, GetBlogPost},
        champs::{ChampsScope, GetChamps},
        cups::{CupsScope, GetCups},
        players::{
            AddSubmitter, GetList, GetPlayers, GetSubmittees, GetSubmitters, PlayersScope,
            RemoveSubmitter, SetSubmitters, UpdateAlias, UpdateBio, UpdatePronouns,
        },
        rankings::{
            GetAverageFinish, GetAverageRankRating, GetCountryRankings,
            GetPersonalRecordWorldRecord, GetTallyPoints, GetTotalTime, RankingsScope,
        },
        regions::{
            GetRegionsAncestors, GetRegionsChildrenTree, GetRegionsDescendants,
            GetRegionsTypeHashmap, GetRegionsWithPlayerCount, RegionsScope,
        },
        scores::{
            GetRecentScores, GetRecords, ScoresScope,
            charts::{ChartsScope, GetCharts, GetChartsDates},
            timesheet::{GetLinechart, GetMatchup, GetTimesheet, TimesheetScope},
        },
        standard_levels::{GetStandardLevels, StandardLevelsScope},
        standards::{GetStandards, StandardsScope},
        submissions::{
            CreateEditSubmission, CreateSubmission, EditEditSubmission, EditSubmission,
            GetEditSubmissionsList, GetSubmissionsList, SubmissionsScope,
        },
        tracks::{GetTracks, TracksScope},
        users::{
            ActivateUser, GetUser, LoginUser, LogoutUser, PasswordForgot, PasswordReset,
            PasswordResetTokenCheck, RegisterUser, UsersScope,
        },
    },
    error::PPResult,
};

use actix_web::web;

mkwpp_api_compat_layer_macros::to_scope!(
    Root: [
        CupsScope: [ GetCups ],
        TracksScope: [ GetTracks ],
        StandardsScope: [ GetStandards ],
        StandardLevelsScope: [ GetStandardLevels ],
        ChampsScope: [ GetChamps ],
        RegionsScope: [
            GetRegionsWithPlayerCount,
            GetRegionsAncestors,
            GetRegionsDescendants,
            GetRegionsTypeHashmap,
            GetRegionsChildrenTree
        ],
        BlogScope: [ GetBlogList, GetBlogPost ],
        PlayersScope: [
            AddSubmitter, GetList, GetPlayers,
            GetSubmittees, GetSubmitters,
            RemoveSubmitter, SetSubmitters,
            UpdateAlias, UpdateBio, UpdatePronouns
        ],
        RankingsScope: [
            GetAverageFinish,
            GetAverageRankRating,
            GetCountryRankings,
            GetPersonalRecordWorldRecord,
            GetTallyPoints,
            GetTotalTime
        ],
        SubmissionsScope: [
            GetSubmissionsList,
            GetEditSubmissionsList,
            CreateSubmission,
            CreateEditSubmission,
            EditSubmission,
            EditEditSubmission
        ],
        UsersScope: [
            ActivateUser, GetUser, LoginUser,
            LogoutUser, PasswordForgot, PasswordReset,
            PasswordResetTokenCheck, RegisterUser
        ],
        ScoresScope: [
            GetRecentScores, GetRecords,
            ChartsScope: [ GetCharts, GetChartsDates ],
            TimesheetScope: [ GetLinechart, GetMatchup, GetTimesheet ]
        ],
    ]
);
