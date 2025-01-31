//! Options to set mqtt client behaviour
use mqtt311::LastWill;
use std::time::Duration;

/// Control how the connection is re-established if it is lost.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReconnectOptions {
    /// Don't automatically reconnect
    Never,
    /// Reconnect automatically if the initial connection was successful.
    ///
    /// Before a reconnection attempt, sleep for the specified amount of time (in seconds).
    AfterFirstSuccess(u64),
    /// Always reconnect automatically.
    ///
    /// Before a reconnection attempt, sleep for the specified amount of time (in seconds).
    Always(u64),
}

/// Client authentication option for mqtt connect packet
#[derive(Clone, Debug)]
pub enum SecurityOptions {
    /// No authentication.
    None,
    /// Use the specified `(username, password)` tuple to authenticate.
    UsernamePassword(String, String),
    #[cfg(feature = "jwt")]
    /// Authenticate against a Google Cloud IoT Core project with the triple
    /// `(project name, private_key.der to sign jwt, expiry in seconds)`.
    GcloudIot(String, Vec<u8>, i64),
}

/// Mqtt through http proxy
#[derive(Clone, Debug)]
pub enum Proxy {
    /// No tunnel
    None,
    /// Tunnel through a proxy using http connect.
    /// (Proxy name, Port, priave_key.der to sign jwt, Expiry in seconds)
    HttpConnect(String, u16, Vec<u8>, i64),
}

/// Mqtt options
#[derive(Clone, Debug)]
pub struct MqttOptions {
    /// broker address that you want to connect to
    broker_addr: String,
    port: u16,
    /// keep alive time to send pingreq to broker when the connection is idle
    keep_alive: Duration,
    /// clean (or) persistent session
    clean_session: bool,
    /// client identifier
    client_id: String,
    /// connection method
    ca: Option<Vec<u8>>,
    client_auth: Option<(Vec<u8>, Vec<u8>)>,
    alpn: Option<Vec<Vec<u8>>>,
    /// proxy
    proxy: Proxy,
    /// reconnection options
    reconnect: ReconnectOptions,
    /// security options
    security: SecurityOptions,
    /// maximum packet size
    max_packet_size: usize,
    /// last will and testament
    last_will: Option<LastWill>,
    /// request (publish, subscribe) channel capacity
    request_channel_capacity: usize,
    /// notification channel capacity
    notification_channel_capacity: usize,
    /// maximum number of outgoing messages per second
    throttle: Option<u64>,
    /// maximum number of outgoing inflight messages
    inflight: usize,
}

impl Default for MqttOptions {
    fn default() -> Self {
        MqttOptions {
            broker_addr: "127.0.0.1".into(),
            port: 1883,
            keep_alive: Duration::from_secs(30),
            clean_session: true,
            client_id: "test-client".into(),
            ca: None,
            client_auth: None,
            alpn: None,
            proxy: Proxy::None,
            reconnect: ReconnectOptions::AfterFirstSuccess(10),
            security: SecurityOptions::None,
            max_packet_size: 256 * 1024,
            last_will: None,
            request_channel_capacity: 10,
            notification_channel_capacity: 10,
            throttle: None,
            inflight: 100,
        }
    }
}

impl MqttOptions {
    /// New mqtt options
    pub fn new<S: Into<String>, T: Into<String>>(id: S, host: T, port: u16) -> MqttOptions {
        // TODO: Validate if addr is proper address type
        let id = id.into();
        if id.starts_with(' ') || id.is_empty() {
            panic!("Invalid client id")
        }

        MqttOptions {
            broker_addr: host.into(),
            port,
            keep_alive: Duration::from_secs(60),
            clean_session: true,
            client_id: id,
            ca: None,
            client_auth: None,
            alpn: None,
            proxy: Proxy::None,
            reconnect: ReconnectOptions::AfterFirstSuccess(10),
            security: SecurityOptions::None,
            max_packet_size: 256 * 1024,
            last_will: None,
            request_channel_capacity: 10,
            notification_channel_capacity: 10,
            throttle: None,
            inflight: 100,
        }
    }

    /// Broker address
    pub fn broker_address(&self) -> (String, u16) {
        (self.broker_addr.clone(), self.port)
    }

    pub fn set_ca(mut self, ca: Vec<u8>) -> Self {
        self.ca = Some(ca);
        self
    }

    pub fn ca(&self) -> Option<Vec<u8>> {
        self.ca.clone()
    }

    pub fn set_client_auth(mut self, cert: Vec<u8>, key: Vec<u8>) -> Self {
        self.client_auth = Some((cert, key));
        self
    }

    pub fn client_auth(&self) -> Option<(Vec<u8>, Vec<u8>)> {
        self.client_auth.clone()
    }

    pub fn set_alpn(mut self, alpn: Vec<Vec<u8>>) -> Self {
        self.alpn = Some(alpn);
        self
    }

    pub fn alpn(&self) -> Option<Vec<Vec<u8>>> {
        self.alpn.clone()
    }

    /// Set number of seconds after which client should ping the broker
    /// if there is no other data exchange
    pub fn set_keep_alive(mut self, secs: u16) -> Self {
        if secs < 5 {
            panic!("Keep alives should be >= 5  secs");
        }

        self.keep_alive = Duration::from_secs(u64::from(secs));
        self
    }

    /// Keep alive time
    pub fn keep_alive(&self) -> Duration {
        self.keep_alive
    }

    /// Client identifier
    pub fn client_id(&self) -> String {
        self.client_id.clone()
    }

    /// Set packet size limit (in Kilo Bytes)
    pub fn set_max_packet_size(mut self, sz: usize) -> Self {
        self.max_packet_size = sz * 1024;
        self
    }

    /// Maximum packet size
    pub fn max_packet_size(&self) -> usize {
        self.max_packet_size
    }

    /// `clean_session = true` removes all the state from queues & instructs the broker
    /// to clean all the client state when client disconnects.
    ///
    /// When set `false`, broker will hold the client state and performs pending
    /// operations on the client when reconnection with same `client_id`
    /// happens. Local queue state is also held to retransmit packets after reconnection.
    pub fn set_clean_session(mut self, clean_session: bool) -> Self {
        self.clean_session = clean_session;
        self
    }

    /// Clean session
    pub fn clean_session(&self) -> bool {
        self.clean_session
    }

    pub fn set_proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = proxy;
        self
    }

    pub fn proxy(&self) -> Proxy {
        self.proxy.clone()
    }

    /// Time interval after which client should retry for new
    /// connection if there are any disconnections. By default, no retry will happen
    pub fn set_reconnect_opts(mut self, opts: ReconnectOptions) -> Self {
        self.reconnect = opts;
        self
    }

    /// Reconnection options
    pub fn reconnect_opts(&self) -> ReconnectOptions {
        self.reconnect
    }

    /// Set security option
    /// Supports username-password auth, tls client cert auth, gcloud iotcore jwt auth
    pub fn set_security_opts(mut self, opts: SecurityOptions) -> Self {
        self.security = opts;
        self
    }

    /// Security options
    pub fn security_opts(&self) -> SecurityOptions {
        self.security.clone()
    }

    /// Set last will and testament
    pub fn set_last_will(mut self, last_will: LastWill) -> Self {
        self.last_will = Some(last_will);
        self
    }

    /// Last will and testament
    pub fn last_will(&self) -> Option<mqtt311::LastWill> {
        self.last_will.clone()
    }

    /// Set notification channel capacity
    pub fn set_notification_channel_capacity(mut self, capacity: usize) -> Self {
        self.notification_channel_capacity = capacity;
        self
    }

    /// Notification channel capacity
    pub fn notification_channel_capacity(&self) -> usize {
        self.notification_channel_capacity
    }

    /// Set request channel capacity
    pub fn set_request_channel_capacity(mut self, capacity: usize) -> Self {
        self.request_channel_capacity = capacity;
        self
    }

    /// Request channel capacity
    pub fn request_channel_capacity(&self) -> usize {
        self.request_channel_capacity
    }

    /// Enables throttling and sets outoing message rate to the specified 'rate'
    pub fn set_throttle(mut self, rate: u64) -> Self {
        if rate == 0 {
            panic!("zero rate is not allowed");
        }

        self.throttle = Some(rate);
        self
    }

    /// Outgoing message rate
    pub fn throttle(&self) -> Option<u64> {
        self.throttle
    }

    /// Set number of concurrent in flight messages
    pub fn set_inflight(mut self, inflight: usize) -> Self {
        if inflight == 0 {
            panic!("zero in flight is not allowed")
        }

        self.inflight = inflight;
        self
    }

    /// Number of concurrent in flight messages
    pub fn inflight(&self) -> usize {
        self.inflight
    }
}

#[cfg(test)]
mod test {
    use crate::mqttoptions::{MqttOptions, ReconnectOptions};

    #[test]
    #[should_panic]
    fn client_id_startswith_space() {
        let _mqtt_opts = MqttOptions::new(" client_a", "127.0.0.1", 1883)
            .set_reconnect_opts(ReconnectOptions::Always(10))
            .set_clean_session(true);
    }

    #[test]
    #[should_panic]
    fn no_client_id() {
        let _mqtt_opts = MqttOptions::new("", "127.0.0.1", 1883)
            .set_reconnect_opts(ReconnectOptions::Always(10))
            .set_clean_session(true);
    }
}
