const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: 'development',
    entry: './src/index.tsx',
    resolve: {
        extensions: ['.js', '.ts', '.tsx', '.jsx']
    },
    module: {
        rules: [
            {
                test: /\.m?(js|jsx|ts|tsx)$/,
                exclude: /(node_modules|bower_components)/,
                use: {
                    // `.swcrc` can be used to configure swc
                    loader: 'swc-loader'
                }
            }
        ]
    },
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(__dirname, 'public/index.html')
        })
    ],
    devServer: {
        liveReload: true
    }
};
