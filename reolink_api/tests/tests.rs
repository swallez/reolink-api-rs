use chrono::TimeDelta;
use reolink_api::*;
use reolink_api::blocking::ReolinkClient;

fn get_client() -> anyhow::Result<ReolinkClient> {
    dotenv::dotenv().ok();

    let url = std::env::var("REOLINK_URL")?;
    let login = std::env::var("REOLINK_LOGIN")?;
    let password = std::env::var("REOLINK_PASSWORD")?;

    ReolinkClient::new(&url, login, password)
}

#[test]
fn test_login() -> anyhow::Result<()> {
    use crate::api::security::login::*;
    let api = get_client()?;

    let resp = api.exec(&LoginRequest::new(
        &std::env::var("REOLINK_LOGIN")?,
        &std::env::var("REOLINK_PASSWORD")?,
    ))?;

    print!("{:?}", resp);
    Ok(())
}

#[test]
fn test_logout() -> anyhow::Result<()> {
    use crate::api::security::logout::*;
    let api = get_client()?;
    // Make sure we have a token
    api.login()?;

    let resp = api.exec(&LogoutRequest{})?;

    print!("{:?}", resp);
    Ok(())
}

#[test]
fn test_get_user() -> anyhow::Result<()> {
    use crate::api::security::get_user::*;
    let api = get_client()?;

    let txt = api.exec_with_details(&GetUserRequest)?;

    print!("{:?}", txt);
    Ok(())
}

#[ignore]
#[test]
fn test_add_user() -> anyhow::Result<()> {
    use crate::api::security::add_user::*;
    let api = get_client()?;

    let txt = api.exec_with_details(&AddUserRequest {
        user: AddUser {
            username: "newuser".to_string(),
            password: "zeechohya5ie8daeLaiy".to_string(),
            level: "admin".to_string(),
        }
    })?;

    print!("{:?}", txt);
    Ok(())
}

#[test]
fn test_rec_request() -> anyhow::Result<()> {
    use crate::api::record::get_recording_v20::*;
    let api = get_client()?;

    let resp = api.exec(&GetRecordingRequest {
        channel: 2
    })?;

    println!("{:#?}", resp);
    Ok(())
}

#[test]
fn test_search() -> anyhow::Result<()> {
    use crate::api::record::search::*;
    let api = get_client()?;

    let date_time = chrono::NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()
        .and_hms_opt(0, 0, 0).unwrap();

    let resp = api.exec(&SearchRequest {
        search: Search {
            channel: 0,
            only_status: false,
            stream_type: "main".to_string(),
            start_time: date_time.into(),
            // start_time: Time {
            //     year: 2024,
            //     mon: 12,
            //     day: 25,
            //     hour: 00,
            //     min: 00,
            //     sec: 00,
            // },
            end_time: (date_time + TimeDelta::days(1) - TimeDelta::seconds(1)).into(),
            // end_time: Time {
            //     year: 2024,
            //     mon: 12,
            //     day: 25,
            //     hour: 23,
            //     min: 59,
            //     sec: 59,
            // }
        }
    })?;

    println!("{:#?}", resp);
    Ok(())
}

#[test]
fn test_nvr_download() -> anyhow::Result<()> {
    use crate::api::record::nvr_download::*;
    let api = get_client()?;

    let date_time = chrono::NaiveDate::from_ymd_opt(2025, 2, 9).unwrap()
        .and_hms_opt(0, 0, 0).unwrap();

    let resp = api.exec(&NvrDownloadRequest {
        nvr_download: NvrDownload {
            channel: 1,
            stream_type: "main".to_string(),
            start_time: date_time.into(),
            end_time: (date_time + TimeDelta::days(1) - TimeDelta::seconds(1)).into(),
        }
    })?;

    println!("{:#?}", resp);
    Ok(())
}

#[test]
fn  test_get_ability() -> anyhow::Result<()> {
    use crate::api::system::get_ability::*;
    let api = get_client()?;

    let resp = api.exec(&GetAbilityRequest {
        user: GetAbility {
            user_name: "NULL".to_string(),
        }
    })?;

    api.logout()?;

    println!("{:#?}", resp);
    Ok(())
}

#[test]
fn test_get_channel_status() -> anyhow::Result<()> {
    use crate::api::system::get_channel_status::*;
    let api = get_client()?;

    let resp = api.exec(&GetChannelStatusRequest)?;

    println!("{:#?}", resp);
    Ok(())
}

#[ignore]
#[test]
fn test_download() -> anyhow::Result<()> {
    use crate::api::record::download::*;
    let api = get_client()?;

    // Pick a file from the result of `search`
    let source = "some-file.mp4";

    let resp = api.download(&DownloadRequest {
        source: source.to_string(),
        output: Some("output.mp4".to_string()),
    })?;

    println!("{:#?}", resp);

    Ok(())
}

#[test]
fn test_snapshot() -> anyhow::Result<()> {
    use crate::api::record::snapshot::*;
    let api = get_client()?;

    let _resp = api.download(&SnapshotRequest {
        channel: 0,
        rs: "0123456789012345".to_string(),
    })?;

    Ok(())
}

#[test]
fn test_dev_info() -> anyhow::Result<()> {
    use crate::api::system::get_dev_info::*;
    let api = get_client()?;

    let resp = api.exec(&GetDevInfoRequest)?;

    println!("{:#?}", resp);

    Ok(())
}
