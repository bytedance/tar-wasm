/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  webpack: (webpackConfig, { isServer }) => {
    // WASM imports are not supported by default. Workaround inspired by:
    // https://github.com/vercel/next.js/issues/29362#issuecomment-1149903338
    // https://github.com/vercel/next.js/issues/32612#issuecomment-1082704675
    return {
      ...webpackConfig,
      experiments: {
        asyncWebAssembly: true,
        layers: true,
      },
      optimization: {
        ...webpackConfig.optimization,
        moduleIds: "named",
      },
      output: {
        ...webpackConfig.output,
        webassemblyModuleFilename: isServer
          ? "./../static/wasm/[modulehash].wasm"
          : "static/wasm/[modulehash].wasm",
      },
    };
  },
};

module.exports = nextConfig;
