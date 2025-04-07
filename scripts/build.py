import os
import shutil
import subprocess

out_dir = './out'
if os.path.exists(out_dir): shutil.rmtree(out_dir)

wasm_bindgen_cmd = [
    'wasm-bindgen',
    '--no-typescript',
    '--target', 'web',
    '--out-dir', './out/',
    '--out-name', 'DepthLudum',
    './target/wasm32-unknown-unknown/release/DepthLudum.wasm'
]

subprocess.run(wasm_bindgen_cmd, check=True)

assets_src = 'assets'
assets_dest = os.path.join(out_dir, 'assets')
shutil.copytree(assets_src, assets_dest)
shutil.rmtree('out/assets/raw')

html_content = """
<!doctype html>
<html lang="en">
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>DepthLudum</title>
    </head>
    <body style="margin: 0px;">
        <script type="module">import init from './DepthLudum.js'; init().catch(console.error);</script>
    </body>
</html>
"""
index_html_dest = os.path.join(out_dir, 'index.html')
with open(index_html_dest, 'w') as html_file: html_file.write(html_content)