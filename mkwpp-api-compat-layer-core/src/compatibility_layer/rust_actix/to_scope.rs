use super::to_route::{ToActixRoute, ExtraInput};
use crate::{
    endpoint::{
        Endpoint, Root, Scope,
        blog::{BlogScope, GetBlogList, GetBlogPost},
        champs::{ChampsScope, GetChamps},
        cups::{CupsScope, GetCups},
        players::{
            AddSubmitter, AdminPlayerDelete, AdminPlayerEdit, AdminPlayerInsert,
            GetAdminPlayerList, GetPlayers, GetPlayersList, GetSubmittees, GetSubmitters,
            PlayersScope, RemoveSubmitter, SetSubmitters, UpdateAlias, UpdateBio, UpdatePronouns,
        },
        rankings::{
            GetAverageFinish, GetAverageRankRating, GetCountryRankings,
            GetPersonalRecordWorldRecord, GetTallyPoints, GetTotalTime, RankingsScope,
        },
        regions::{
            GetRegionsAncestors, GetRegionsChildrenTree, GetRegionsDescendants,
            GetRegionsTypeHashmap, GetRegionsWithPlayerCount, RegionsAdminDelete, RegionsAdminEdit,
            RegionsAdminInsert, RegionsScope,
        },
        scores::{
            AdminScoreDelete, AdminScoreEdit, AdminScoreInsert, GetAdminScore, GetAdminScoreList,
            GetRecentScores, GetRecords, ScoresScope,
            charts::{ChartsScope, GetCharts, GetChartsDates},
            timesheet::{GetLinechart, GetMatchup, GetTimesheet, TimesheetScope},
        },
        standard_levels::{GetStandardLevels, StandardLevelsScope},
        standards::{GetStandards, StandardsScope},
        submissions::{
            CreateEditSubmission, CreateSubmission, EditEditSubmission, EditSubmission,
            EditSubmissionDelete, GetEditSubmissionsList, GetSubmissionsList, SubmissionDelete,
            SubmissionsScope,
        },
        tracks::{GetTracks, TracksScope},
        users::{
            ActivateUser, AdminUserDelete, AdminUserEdit, AdminUserInsert, GetAdminUserList,
            GetUser, IsAdmin, LoginUser, LogoutUser, PasswordForgot, PasswordUpdate,PasswordReset,
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
            GetRegionsChildrenTree,
            RegionsAdminEdit, RegionsAdminDelete, RegionsAdminInsert
        ],
        BlogScope: [ GetBlogList, GetBlogPost ],
        PlayersScope: [
            AddSubmitter, GetPlayersList, GetPlayers,
            GetSubmittees, GetSubmitters,
            RemoveSubmitter, SetSubmitters,
            UpdateAlias, UpdateBio, UpdatePronouns,
            GetAdminPlayerList,
            AdminPlayerInsert, AdminPlayerEdit, AdminPlayerDelete
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
            EditEditSubmission,
            EditSubmissionDelete, SubmissionDelete
        ],
        UsersScope: [
            ActivateUser, GetUser, LoginUser,
            LogoutUser, PasswordForgot, PasswordUpdate,PasswordReset,
            PasswordResetTokenCheck, RegisterUser, IsAdmin,
            AdminUserEdit, AdminUserDelete, AdminUserInsert, GetAdminUserList
        ],
        ScoresScope: [
            GetRecentScores, GetRecords,
            AdminScoreEdit, AdminScoreDelete, AdminScoreInsert, GetAdminScoreList, GetAdminScore,
            ChartsScope: [ GetCharts, GetChartsDates ],
            TimesheetScope: [ GetLinechart, GetMatchup, GetTimesheet ]
        ],
    ]
);
