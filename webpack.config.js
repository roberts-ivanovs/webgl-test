const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: {
    'frontend/js/dist/containers/index':
      './frontend/static/frontend/js/src/App.tsx',
  },
  output: {
    path: '/opt/services/djangoapp/static/',
    publicPath: '/static/',
  },
  devtool: 'inline-source-map',
  mode: 'development',
  module: {
    rules: [
      {
        resolve: {
          extensions: ['.js', '.jsx', '.ts', '.tsx'],
        },
        test: /\.(js|jsx|tsx|ts)$/,
        exclude: /node_modules/,
        use: {
          loader: 'ts-loader',
        },
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\.s[ac]ss$/i,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              modules: true,
              sourceMap: true,
              importLoaders: 2,
            },
          },
          'sass-loader',
        ],
      },
    ],
  },
  resolve: {
    alias: {
      jquery: 'jquery/src/jquery',
    },
  },

  plugins: [
    new MiniCssExtractPlugin({
      filename: 'core/style/style.css',
      chunkFilename: '[name][hash].css',
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};
