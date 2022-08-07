const WindiCSSWebpackPlugin = require('windicss-webpack-plugin')

module.exports = (config, { mode }) => {
  if (mode === 'development') {
    // Add dev plugin
  }

  // Add custom rules for your project
  // config.module.rules.push(YOUR_RULE)

  config.plugins.push(new WindiCSSWebpackPlugin())

  return config
}
