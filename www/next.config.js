module.exports = {
    webpack: (config, options) => {
        config.experiments = {
            asyncWebAssembly: true
        }

        // Important: return the modified config
        return config
      },
}