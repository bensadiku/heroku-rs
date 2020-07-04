extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::apps;
use heroku_rs::endpoints::builds;
use heroku_rs::endpoints::domains;
use heroku_rs::endpoints::dynos;
use heroku_rs::endpoints::formations;
use heroku_rs::endpoints::releases;
use heroku_rs::endpoints::slugs;
use heroku_rs::framework::apiclient::HerokuApiClient;
use std::collections::HashMap;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = "heroku-rs-tests";

    // create_app(api_client, app_name);
    // delete_app(api_client, app_name); // Careful here :)
    // patch_app(api_client, app_name);
    get_app(api_client, app_name);
    // get_app_raw_response(api_client, app_name);
    // list_apps(api_client);
    // list_account_apps(api_client);
    // get_dyno(api_client, app_name);
    // list_dynos(api_client, app_name);
    // create_dyno_simple(api_client, app_name);
    // create_dyno_complex(api_client, app_name);
    // list_dynos(api_client, app_name);
    // restart_dyno(api_client);
    // restart_all_dynos(api_client, app_name);
    // get_dyno_size_list(api_client);
    // get_dyno_size_details(api_client);

    // enable_app_acm(api_client, app_name);
    // disable_app_acm(api_client, app_name);
    // refresh_app_acm(api_client, app_name);
    // get_app_features(api_client, app_name);
    // get_app_feature(api_client, app_name);
    // patch_app_feature(api_client, app_name);

    // create_app_webhook(api_client, app_name);
    // get_app_webhooks(api_client, app_name);
    // get_app_webhook(api_client, app_name);
    // patch_app_webhook(api_client, app_name);
    // delete_app_webhook(api_client, app_name);

    // get_app_webhook_delivery(api_client, app_name);
    // get_app_webhook_deliveries(api_client, app_name);

    // get_app_webhook_event(api_client, app_name);
    // get_app_webhook_events(api_client, app_name);

    // create_app_build(api_client, app_name);
    // get_app_builds(api_client, app_name);
    // get_app_build(api_client, app_name);
    // delete_app_build(api_client, app_name);

    // update_buildpack_installation(api_client, app_name);
    // get_buildpack_installations(api_client, app_name);

    // create_app_domain(api_client, app_name);
    // get_app_domains(api_client, app_name);
    // get_app_domain(api_client, app_name);
    // delete_app_domain(api_client, app_name);

    // dyno_action_stop(api_client, app_name);

    // get_app_formation(api_client, app_name);
    // list_app_formations(api_client, app_name);
    // update_app_formation(api_client, app_name);

    // create_slug(api_client, app_name);
    // get_slug(api_client, app_name);
    // get_app_release(api_client, app_name, "4");
    // list_app_releases(api_client, app_name);
    // create_app_release(api_client, app_name);
    // rollback_app_release(api_client, app_name);

    // create_app_setup(api_client);
    // get_app_setup(api_client);

    // create_app_sni(api_client, app_name);
    // get_app_sni(api_client, app_name);
    // get_app_sni_list(api_client, app_name);
    // delete_app_sni(api_client, app_name);
    // update_app_sni(api_client, app_name);

    // get_app_ssl_list(api_client, app_name);
    // get_app_ssl(api_client, app_name);
    // delete_app_ssl(api_client, app_name); // Careful here.
    // create_app_ssl(api_client, app_name);
    // update_app_ssl(api_client, app_name);
}

// get a specific webhook event
fn get_app_webhook_event<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let event_id = "123";
    let response = api_client.request(&apps::WebhookEventDetails::new(app_id, event_id));
    print_response(response);
}

// get webhook events list
fn get_app_webhook_events<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&apps::WebhookEventList::new(app_id));
    print_response(response);
}

// Update app SSL
fn update_app_ssl<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let ssl_id = "123";
    // `new` method takes only the required parameters
    let update_ssl = &apps::SSLUpdate::new(app_id, ssl_id)
        .certificate_chain("chain_here")
        .private_key("key_here")
        .build();
    let response = api_client.request(update_ssl);
    print_response(response);
}

// Create app SSL
fn create_app_ssl<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let certificate_chain = "chain_here";
    let private_key = "key_here";
    // `new` method takes only the required parameters
    let response = api_client.request(&apps::SSLCreate::new(
        app_id,
        certificate_chain,
        private_key,
    ));
    print_response(response);
}

// delete app SSL
fn delete_app_ssl<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let ssl_id = "123";
    let response = api_client.request(&apps::SSLDelete::new(app_id, ssl_id));
    print_response(response);
}

// get app SSL
fn get_app_ssl<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let ssl_id = "123";
    let response = api_client.request(&apps::SSLDetails::new(app_id, ssl_id));
    print_response(response);
}

// get app SSL list
fn get_app_ssl_list<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&apps::SSLList::new(&app_id));
    print_response(response);
}

// update app SNI list
fn update_app_sni<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let certificate_chain = "chain_here";
    let private_key = "key_here";
    let sni_id = "123";
    let response = api_client.request(&apps::SNIUpdate::new(
        app_id,
        sni_id,
        certificate_chain,
        private_key,
    ));
    print_response(response);
}

// delete app SNI list
fn delete_app_sni<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let sni_id = "123";
    let response = api_client.request(&apps::SNIDelete::new(app_id, sni_id));
    print_response(response);
}

// get app SNI list
fn get_app_sni_list<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&apps::SNIList::new(app_id));
    print_response(response);
}

// get app SNI
fn get_app_sni<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let sni_id = "123";
    let response = api_client.request(&apps::SNIDetails::new(app_id, sni_id));
    print_response(response);
}

// create app SNI
fn create_app_sni<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let certificate_chain = "chain_here";
    let private_key = "key_here";
    let response = api_client.request(&apps::SNICreate::new(
        app_id,
        certificate_chain,
        private_key,
    ));
    print_response(response);
}

// get info about a app setup
fn get_app_setup<T: HerokuApiClient>(api_client: &T) {
    let setup_id = "APP_SETUP_ID";
    let response = api_client.request(&apps::AppSetupDetails::new(setup_id));
    print_response(response);
}

// create app setup
fn create_app_setup<T: HerokuApiClient>(api_client: &T) {
    let source_blob_url = "https://github.com/heroku/ruby-rails-sample/tarball/master/";
    let new_app_setup = &apps::AppSetupCreate::new(source_blob_url)
        .locked(true)
        .name("gotye-probably")
        .build();
    let response = api_client.request(new_app_setup);
    print_response(response);
}

// get info about a slug
fn get_slug<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let slug_id = "SLUG_ID";
    let response = api_client.request(&slugs::SlugDetails { app_id, slug_id });
    print_response(response);
}

/// create a slug
fn create_slug<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let mut process_types = HashMap::new();
    process_types.insert("web", "./bin/web -p $PORT");

    let response = api_client.request(
        &slugs::SlugCreate::new(app_id, process_types)
            .commit("60883d9e8947a57e04dc9124f25df004866a2051")
            .commit_description("fixed a bug with API documentation")
            .buildpack_provided_description("Ruby/Rack")
            .checksum("SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            .stack("cedar-14")
            .build(),
    );

    print_response(response);
}

/// Stop dyno
fn dyno_action_stop<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let domain_id = "DYNO_ID_OR_NAME";
    let response = api_client.request(&domains::DomainDelete { app_id, domain_id });
    print_response(response);
}

/// Delete domain
fn delete_app_domain<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let domain_id = "DOMAIN_ID_OR_HOSTNAME";
    let response = api_client.request(&domains::DomainDelete { app_id, domain_id });
    print_response(response);
}

/// Get domain
fn get_app_domain<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let domain_id = "DOMAIN_ID_OR_HOSTNAME";
    let response = api_client.request(&domains::DomainDetails { app_id, domain_id });
    print_response(response);
}

/// Get domains list
fn get_app_domains<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&domains::DomainList { app_id });
    print_response(response);
}

/// Create domain
fn create_app_domain<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let hostname = "heroku-rs.tests.com";
    let response = api_client.request(&domains::DomainCreate::new(app_id, hostname));
    print_response(response);
}

/// Get a list of build pack installations
fn get_buildpack_installations<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&builds::BuildPackInstallationList::new(app_id));
    print_response(response);
}

/// Update build pack installations
fn update_buildpack_installation<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let buildpack_ruby = "https://github.com/heroku/heroku-buildpack-ruby";
    let buildpack_python = "https://github.com/heroku/heroku-buildpack-python";

    let builpack_list = vec![buildpack_ruby, buildpack_python];
    let response = api_client.request(&builds::BuildpackInstallationUpdate::new(
        app_id,
        builpack_list,
    ));
    print_response(response);
}

/// Delete build cache
fn delete_app_build<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&builds::BuildDelete { app_id });
    print_response(response);
}

/// Gets info about a specific build
fn get_app_build<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let build_id = "Build_ID";
    let response = api_client.request(&builds::BuildDetails { app_id, build_id });
    print_response(response);
}

/// Gets a list of builds
fn get_app_builds<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&builds::BuildList { app_id });
    print_response(response);
}

/// Create a new build
fn create_app_build<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    // `new` method takes only the required parameters
    let blob_url = "https://example.com/source.tgz?token=xyz";
    let response = api_client.request(
        &builds::BuildCreate::new(app_name, blob_url)
            .checksum("SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            .version("v1.3.0")
            .buildpack(
                "heroku/ruby",
                "https://github.com/heroku/heroku-buildpack-ruby",
            )
            .build(),
    );
    print_response(response);
}

/// Gets a list of webhook deliveries.
fn get_app_webhook_deliveries<T: HerokuApiClient>(api_client: &T, app_name: &str) {
    let response = api_client.request(&apps::AppWebhookDeliveryList { app_id: app_name });
    print_response(response);
}

/// Gets details about a specific webhook delivery.
fn get_app_webhook_delivery<T: HerokuApiClient>(api_client: &T, app_name: &str) {
    let webhook_id = "WEBHOOK_DELIVERY_ID";
    let response = api_client.request(&apps::AppWebhookDetails::new(app_name, webhook_id));

    print_response(response);
}

/// Patch a specific webhook.
fn patch_app_webhook<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let webhook_id = "WEBHOOK_ID";
    let update_app_webhook = &apps::AppWebhookUpdate::new(app_name, webhook_id)
        .include(vec!["api:release"])
        .level("notify")
        .url("https://www.bing.com")
        .build();

    let response = api_client.request(update_app_webhook);
    print_response(response);
}

/// Gets details about a specific webhook.
fn get_app_webhook<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let webhook_id = "WEBHOOK_ID";
    let response = api_client.request(&apps::AppWebhookDetails::new(app_name, webhook_id));
    print_response(response);
}

/// Gets a list of all webhooks that are available in the specified app.
fn get_app_webhooks<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let response = api_client.request(&apps::AppWebhookList::new(app_name));
    print_response(response);
}

/// Delete a specific app webhook by id
fn delete_app_webhook<T: HerokuApiClient>(api_client: &T, app_name: &str) {
    let webhook_id = "WEBHOOK_ID";
    let response = api_client.request(&apps::AppWebhookDelete::new(app_name, webhook_id));
    print_response(response);
}

/// Create a new app webhook
fn create_app_webhook<T: HerokuApiClient>(api_client: &T, app_name: &str) {
    let webhook = &apps::AppWebhookCreate::new(
        app_name,
        vec!["api:release"],
        "notify",
        "https://www.google.com",
    );

    let response = api_client.request(webhook);
    print_response(response);
}

fn patch_app_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let update_feature = &apps::AppFeatureUpdate::new(app_name, "spaces-dns-discovery", false);
    let response = api_client.request(update_feature);
    print_response(response);
}

fn get_app_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let response = api_client.request(&apps::AppFeatureDetails {
        app_id: app_name,
        feature_id: "spaces-dns-discovery",
    });
    print_response(response);
}

fn get_app_features<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let response = api_client.request(&apps::AppFeatureList { app_id: app_name });
    print_response(response);
}

fn refresh_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let response = api_client.request(&apps::AppRefreshAcm::new(app_name));
    print_response(response);
}

fn disable_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let response = api_client.request(&apps::AppDisableAcm { app_id: &app_name });
    print_response(response);
}

fn enable_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: &str) {
    let response = api_client.request(&apps::AppEnableAcm::new(app_name));
    print_response(response);
}

fn patch_app<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let patch = &apps::AppUpdate::new(app_id)
        .name("cool-name")
        .maintenance(false)
        .build();
    let response = api_client.request(patch);
    print_response(response);
}

fn delete_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let response = api_client.request(&apps::AppDelete::new(app_id));
    print_response(response);
}

fn create_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let new_app = &apps::AppCreate::new().name(app_id).build();
    let response = api_client.request(new_app);
    print_response(response);
}

fn get_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let response = api_client.request(&apps::AppDetails::new(app_id));
    print_response(response);
}

fn get_app_test<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let response = apps::AppDetails::new(app_id).get(api_client);
    print_response(response);
}

fn get_app_raw_response<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    // If successful, this returns a raw reqwest::blocking::response, do whatever with it!
    let response = api_client.request_raw(&apps::AppDetails::new(app_id));
    match response {
        Ok(res) => println!("Ok: {:?}", res),
        Err(e) => println!("Error: {}", e),
    }
}

fn list_account_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = "my-heroku-email@here.io";
    let resp = api_client.request(&apps::AccountAppList::new(account_id));
    print_response(resp);
}

fn list_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let resp = api_client.request(&apps::AppList {});
    print_response(resp);
}

fn get_dyno_size_list<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&dynos::DynoSizeList {});
    print_response(response);
}

fn get_dyno_size_details<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let dyno_size_id = "Shield-L";
    let response = api_client.request(&dynos::DynoSizeDetails {
        size_id: dyno_size_id,
    });
    print_response(response);
}

fn get_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let dyno_id = "web.1";

    let response = api_client.request(&dynos::DynoDetails { app_id, dyno_id });
    print_response(response);
}

fn list_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let resp = api_client.request(&dynos::DynoList { app_id });
    print_response(resp);
}

fn create_dyno_simple<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let response = api_client.request(&dynos::DynoCreate::new(app_id, "bash"));

    print_response(response);
}

fn create_dyno_complex<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let mut custom_env_vars = HashMap::new();

    custom_env_vars.insert("COLUMNS", "80");

    custom_env_vars.insert("LINES", "24");

    let new_dyno_complex = &dynos::DynoCreate::new(app_id, "bash")
        .attach(true)
        .env(custom_env_vars)
        .build();

    let response = api_client.request(new_dyno_complex);

    print_response(response);
}

fn restart_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_id = "heroku-rs-tests";
    let dyno_id = "web.1";

    let resp = api_client.request(&dynos::DynoRestart { app_id, dyno_id });
    print_response(resp);
}

fn restart_all_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let resp = api_client.request(&dynos::DynoAllRestart { app_id });
    print_response(resp);
}

fn list_app_formations<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let resp = api_client.request(&formations::FormationList { app_id });
    print_response(resp);
}

fn get_app_formation<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let resp = api_client.request(&formations::FormationDetails {
        app_id,
        formation_id: "web",
    });
    print_response(resp);
}

fn update_app_formation<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let resp = api_client.request(
        &formations::FormationUpdate::new(app_id, "web")
            .quantity(2)
            .size("standard-1X")
            .build(),
    );
    print_response(resp);
}

fn list_app_releases<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let resp = api_client.request(&releases::ReleaseList { app_id });
    print_response(resp);
}

fn get_app_release<T: HerokuApiClient>(api_client: &T, app_id: &str, release_id: &str) {
    let resp = api_client.request(&releases::ReleaseInfo { app_id, release_id });
    print_response(resp);
}

fn create_app_release<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let resp = api_client.request(
        &releases::ReleaseCreate::new(app_id, "2dbce013-4be8-44e1-8221-c9c74e45949c")
            .description("added new feature")
            .build(),
    );
    print_response(resp);
}

fn rollback_app_release<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: &str) {
    let release_to_rollback = "v17";
    let resp = api_client.request(&releases::ReleaseRollback::new(app_id, release_to_rollback));
    print_response(resp);
}
