import os
import shutil
import subprocess

out_dir = './out'
if os.path.exists(out_dir): shutil.rmtree(out_dir)

subprocess.run(['cargo', 'build', '--release', '--target', 'wasm32-unknown-unknown'], check=True)
subprocess.run(['cargo', 'build', '--release'], check=True)

wasm_bindgen_cmd = [
    'wasm-bindgen',
    '--no-typescript',
    '--target', 'web',
    '--out-dir', './out/web/',
    '--out-name', 'DepthLudum',
    './target/wasm32-unknown-unknown/release/DepthLudum.wasm'
]

subprocess.run(wasm_bindgen_cmd, check=True)

assets_src = 'assets'
shutil.copytree(assets_src, os.path.join(out_dir, 'web/assets'))
shutil.copytree(assets_src, os.path.join(out_dir, 'win/assets'))
shutil.copy('./target/release/DepthLudum.exe', './out/web/DepthLudum.exe')
shutil.rmtree('out/web/assets/raw')
shutil.rmtree('out/win/assets/raw')

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