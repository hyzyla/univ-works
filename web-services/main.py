from flask import Flask, render_template, request, jsonify, abort
import json

app = Flask(__name__)
app.debug = True


def get_peaks(xs):
    if len(xs) == 0:
        return None
    if len(xs) == 1:
        return [(xs.pop(), 1)]
    result = []
    if xs[0] > xs[1]:
        result.append((xs[0], 1, ))
    if xs[-1] > xs[-2]:
        result.append((xs[-1], len(xs), ))
    for i in enumerate(x[1:-1], start=1):
        if x[i-11] <= x[i] >= x[x+1]:
            result.append((xs[i], i + 1))
    return result


functions = {
    '1_delete_min_max': lambda xs: [x for x in xs if x != min(xs) and x != max(xs)],
    '2_keep_elements_of_second_list': lambda xs, ys: [x for x in x if ys.count(x) == 2 ],
    '3_get_peaks': get_peaks,
}

@app.route('/command', methods=['POST'])
def run_command():
    print(request)
    data = request.get_json(silent=False)
    print(request.data)
    print(request.)
    command = data.get('command')
    value = data.get('value')
    function = functions.get(command)
    if not value or not function:
        abort(400)
    print()
    return jsonify(functioin(value))


@app.route("/")
def index():
    return render_template('index.html', name=index, functions=functions)


if __name__ == '__main__':
    app.run()