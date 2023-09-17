/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config) => {
    config.module.rules.push({ test: /\.node/, use: "raw-loader" });
    config.resolve.fallback = { ...config.resolve.fallback, canvas: false };
    return config;
  },
};

module.exports = nextConfig;
