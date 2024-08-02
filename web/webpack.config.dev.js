const fs = require('fs');
const path = require('path');
const webpack = require('webpack');
const HtmlWebPackPlugin = require('html-webpack-plugin');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const babelOptions = {
    'presets': [
        '@babel/preset-react'
    ],
    'plugins': [ '@babel/plugin-syntax-dynamic-import' ]
};
const isDev = process.env.NODE_ENV === 'development';

const plugins = [
    new webpack.ProvidePlugin({
        process: 'process/browser.js',
        Buffer: [ 'buffer', 'Buffer' ],
    }),
    new webpack.HotModuleReplacementPlugin(),
    new HtmlWebPackPlugin({
        title: 'DataProcessCenter',
        template: path.resolve(__dirname, './index.html'),
        filename: './index.html'
    }),
    new MiniCssExtractPlugin({
        filename: 'css/[name].[hash:8].css',
        chunkFilename: 'css/chunk.[id].[chunkhash:8].css',
    }),
];

if (isDev) {
    plugins.push(new ForkTsCheckerWebpackPlugin({ async: true }));
}

const styleLoader = isDev ?
    { loader: "style-loader" }
    :
    {
        loader: MiniCssExtractPlugin.loader,
        options: {}
    };

module.exports = {
    mode: 'development',
    entry: {
        main: "./src/main.tsx"
    },
    output: {
        path: path.resolve(__dirname, 'docs/'),
        filename: 'js/[name].[hash:8].js',
        chunkFilename: 'js/chunk.[name].[chunkhash:8].js',
        globalObject: 'this'
    },
    resolve: {
        alias: {

        },
        extensions: [
            '.ts', '.tsx', '.js', '.jsx', '.json', '.css', '.scss'
        ],
        fallback: {
            fs: false,
            path: false,
            crypto: false,
        }
    },
    module: {
        rules: [
            {
                test: /\.worker\.(ts|js)$/, // ts结尾,这也很重要
                use: {
                    loader: 'worker-loader',
                    options: {
                        filename: '[name].[hash:8].js', // 打包后chunk的名称
                        chunkFilename: '[name].[hash:8].js', // 打包后chunk的名称
                        inline: "no-fallback", // 开启内联模式,免得爆缺少标签或者跨域的错误
                    }
                }
            },
            {
                test: /\.(md)$/,
                use: 'raw-loader'
            },
            {
                test: /\.demo.tsx?$/,
                use: [
                    {
                        loader: 'babel-loader',
                        options: babelOptions
                    },
                    {
                        loader: 'ts-loader',
                        options: { happyPackMode: true, configFile: "tsconfig.json" }
                    },
                    // {
                    //     loader: "./webpack.demo.loader"
                    // },
                ]
            },
            {
                test: [ /\.tsx?$/ ],
                use: [
                    {
                        loader: 'babel-loader',
                        options: babelOptions
                    },
                    {
                        loader: 'ts-loader',
                        options: { happyPackMode: true, configFile: "tsconfig.json" }
                    }
                ]
            },
            {
                test: /\.jsx?$/,
                loader: 'babel-loader',
                exclude: /node_modules/,
                options: babelOptions
            },
            {
                test: /\.css$/,
                use: [
                    styleLoader,
                    {
                        loader: 'css-loader', // 将 CSS 转化成 CommonJS 模块
                        options: {
                            importLoaders: 1,
                            modules: false
                        }
                    }
                ]
            },
            {
                test: /\.wasm$/,
                type: "javascript/auto"
            },
            {
                test: /\.hdr$/,
                type: "javascript/auto"
            },
            {
                test: /\.mjs$/,
                include: /node_modules/,
                type: "javascript/auto"
            },
            {
                test: /\.(png|jpg|gif|wasm|ttf|hdr|svg)$/,
                use: [
                    {
                        loader: 'url-loader',
                        options: {
                            name: 'assets/[name].[hash:16].[ext]',
                            limit: 1
                        }
                    }
                ]
            },
            {
                enforce: 'pre',
                test: /\.js$/,
                loader: 'source-map-loader',
                exclude: /node_modules/,
                options: {
                    filterSourceMappingUrl: (url, resourcePath) => {
                        if (resourcePath.indexOf("monaco-editor") >= 0 || url.indexOf("monaco-editor") >= 0) {
                            return false;
                        }
                        return true;
                    },
                }
            }
        ]
    },
    // 调试服务
    devServer: {
        compress: true,
        port: 3355,
        host: '0.0.0.0',
        hot: true,
        historyApiFallback: {
            index: '/index.html'
        },
        headers: {
            'Cross-Origin-Embedder-Policy': 'require-corp',
            'Cross-Origin-Opener-Policy': 'same-origin',
        }
    },
    plugins,
    cache: {
        type: 'filesystem', // 使用文件缓存
        allowCollectingMemory: true,
    },
};
