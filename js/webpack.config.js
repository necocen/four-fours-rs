const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: 'development',
    entry: './src/index.tsx',
    output: {
        filename: "index.js",
        path: path.resolve(__dirname, "build"),
    },
    resolve: {
        extensions: ['.mjs', '.js', '.ts', '.tsx', '.jsx']
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: {
                    loader: 'ts-loader'
                }
            }
        ]
    },
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true,
        topLevelAwait: true
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(__dirname, 'public/index.html'),
            scriptLoading: "blocking"
        })
    ],
    devServer: {
        liveReload: true,
        headers: {
            "Cross-Origin-Opener-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp"
        }
    },
    ignoreWarnings: [
        /Circular dependency between chunks with runtime/
    ],
    target: ["web", "es5"]
};
