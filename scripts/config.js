const config = {
  KV_NAMESPACE_ID: "e994fdd7add54bee9b3e905bbad1ce61",
  ACCOUNT_ID: "f1c2ab938f91fa76af879185e872f215",
  WORKER_NAME: "blog-dev",
  ...process.env,
};

if (
  ![config.KV_NAMESPACE_ID, config.ACCOUNT_ID, config.CF_TOKEN].every(Boolean)
) {
  console.error(
    "One of `KV_NAMESPACE_ID`, `ACCOUNT_ID`, `CF_TOKEN` is not set."
  );
  process.exit(1);
}

module.exports = config;
