import os
from flask import Flask, request, jsonify, send_file, Response
import magic
import lib
app = Flask(__name__)


@app.route('/')
def hello_world():
    return 'hello, world v2'


@app.route('/upload', methods=['POST'])
def upload_multipart():

    if 'file' not in request.files:
        return jsonify({"message": "need `file`"})

    file = request.files["file"]
    mime = magic.from_buffer(file.read(2048), mime=True)

    mime_separate = mime.find("/")
    main_type = mime[:mime_separate]
    sub_type = mime[mime_separate+1:]

    if main_type == "image":
        file_id = lib.process_image(sub_type, file)
    else:
        return jsonify({"message": f"unsupported file-type: {mime}"}), 405

    return jsonify({"message": "success", "id": file_id, "main": main_type, "sub": sub_type}), 200


@app.route('/preview/<path:p>', methods=["GET"])
def view_with_path(p):
    path = f"storage/{p}"
    if not os.path.exists(path):
        return Response(), 404

    mime = magic.from_file(path, mime=True)

    return send_file(
        path,
        as_attachment=False,
        download_name=p,
        mimetype=mime
    )


if __name__ == '__main__':
    app.run(host="0.0.0.0", port=4040, debug=True)
