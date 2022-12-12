require('file-loader?name=[name].[ext]!../static/index.html');
require('file-loader?name=[name].[ext]!../static/favicon.ico');
import css from 'xterm/css/xterm.css';
import 'xterm/lib/xterm.js';
import("../pkg/index.js").catch(console.error);
