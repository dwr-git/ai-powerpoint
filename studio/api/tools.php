<?php
// studio/api/tools.php — что помощник AI-PowerPoint знает о себе и какие действия ему разрешены.
//
// Инструменты выполняет БРАУЗЕР (studio.js): только он видит запущенный
// редактор. Сервер лишь объявляет модели, какие инструменты есть, и решает,
// какие из них доступны в текущем режиме.

/** Инструменты, которые ничего не меняют в презентации. Доступны в обоих режимах. */
function studio_read_tools(): array
{
    return [
        [
            'type' => 'function',
            'function' => [
                'name' => 'find_commands',
                'description' => 'Найти команды редактора по смыслу (по-русски или по-английски). '
                    . 'Возвращает id команды, русское название, путь на ленте, горячую клавишу и описание параметров. '
                    . 'Вызывай перед run_command, если не знаешь точный id: id нельзя угадывать.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'query' => ['type' => 'string', 'description' => 'Что нужно сделать, 1–4 слова: «новый слайд», «переход», «анимация», «тема», «заметки», «transition».'],
                    ],
                    'required' => ['query'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'get_state',
                'description' => 'Текущее состояние презентации: имя, сохранена ли, размер слайдов, тема, список слайдов '
                    . '(номер, заголовок, макет, переход, число анимаций, начало заметок), текущий слайд и выделение.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'read_slides',
                'description' => 'Прочитать текст презентации: заголовки, текст и заметки докладчика всех слайдов (или части). '
                    . 'Нужен, чтобы пересказать, проверить, найти или переписать.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'from' => ['type' => 'integer', 'description' => 'С какого слайда (с 1), по умолчанию 1'],
                        'count' => ['type' => 'integer', 'description' => 'Сколько слайдов, по умолчанию все'],
                    ],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'slide_details',
                'description' => 'Всё о слайде: фигуры с id, видами, положением и размером (пт), текстом и заливкой; анимации, переход, заметки. '
                    . 'id фигур нужны для text.set, selection.select, shape.move и т. п.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'slide' => ['type' => 'integer', 'description' => 'Номер слайда (с 1); по умолчанию текущий'],
                    ],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'look_at_screen',
                'description' => 'Сделать снимок экрана редактора и посмотреть на него: проверить результат или увидеть, как выглядит слайд.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
    ];
}

/** Инструменты, которые меняют презентацию. Только в режиме «Сделай». */
function studio_act_tools(): array
{
    return [
        [
            'type' => 'function',
            'function' => [
                'name' => 'run_command',
                'description' => 'Выполнить команду редактора с параметрами. Диалогов нет: всё задаётся параметрами. '
                    . 'Команды форматирования действуют на выделение (текст или фигуры). Действие отменяется (Ctrl+Z).',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'id' => ['type' => 'string', 'description' => 'id команды, например slide.new, text.set, slide.notes, transition.set, design.theme'],
                        'params' => ['type' => 'object', 'description' => 'Параметры команды (JSON-объект). Можно не передавать — тогда значения по умолчанию.'],
                    ],
                    'required' => ['id'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'add_slides',
                'description' => 'Добавить несколько слайдов одним шагом после текущего: у каждого заголовок, пункты и, если нужно, заметки докладчика. '
                    . 'Удобно, чтобы собрать презентацию по плану или из текста.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'slides' => [
                            'type' => 'array',
                            'description' => 'Список слайдов: [{title, bullets: ["пункт", …], notes?, layout?: "titleAndContent"|"title"|"sectionHeader"|"twoContent"|"titleOnly"|"blank"}]',
                            'items' => ['type' => 'object'],
                        ],
                    ],
                    'required' => ['slides'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'undo',
                'description' => 'Отменить последние действия (как Ctrl+Z).',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'steps' => ['type' => 'integer', 'description' => 'Сколько шагов отменить, по умолчанию 1', 'minimum' => 1, 'maximum' => 20],
                    ],
                ],
            ],
        ],
    ];
}

/** Системная инструкция. $mode: 'advice' (Совет) или 'act' (Сделай). */
function studio_system_prompt(string $mode): string
{
    $available = $mode === 'act' ? array_merge(studio_read_tools(), studio_act_tools()) : studio_read_tools();
    $catalog = studio_tool_catalog($available);

    $common = <<<TXT
Ты — встроенный помощник AI-PowerPoint, редактора презентаций, устроенного как Microsoft PowerPoint: лента (Главная, Вставка,
Рисование, Дизайн, Переходы, Анимация, Слайд-шоу, Рецензирование, Вид), макеты и образцы слайдов, темы, фигуры, таблицы,
диаграммы, SmartArt, заметки докладчика, показ слайдов; интерфейс на русском.
Презентации — .pptx (файлы пользователя хранятся на его сервере, «Мои файлы»).
Пользователь работает в редакторе прямо сейчас. К его сообщению приложено состояние презентации, а иногда снимок экрана.

Как отвечать:
- Всегда по-русски, коротко и по делу: 1–6 предложений или короткий нумерованный список шагов.
- Пункты ленты пиши так: «Главная › Слайды › Создать слайд». Горячие клавиши — в скобках: (Ctrl+M). Сочетания — как в PowerPoint.
- Названия команд бери из find_commands, а не по памяти. Не выдумывай функций, которых нет.
- Координаты и размеры в командах — в пунктах (pt) от левого верхнего угла слайда; слайд 16:9 — 960×540 пт.
- Номера слайдов для пользователя — с 1; в параметрах команд (index) — с 0.
- Сохранение: Ctrl+S сохраняет презентацию в «Мои файлы» на сервере (значок папки в панели помощника).
- Хороший слайд: короткий заголовок, 3–6 пунктов по одной мысли, без длинных абзацев; подробности — в заметки докладчика.

Твои инструменты (других нет — ни computer_use, ни кликов по координатам):
{$catalog}
Вызывай их штатно. Если штатный вызов недоступен, напиши вызов текстом, по одному блоку на вызов:
<tool_call><function=ИМЯ><parameter=КЛЮЧ>ЗНАЧЕНИЕ</parameter></function></tool_call>
Значения-объекты и списки пиши в JSON. После блока ничего не пиши — дождись результата.
TXT;

    if ($mode === 'act') {
        return $common . "\n\n" . <<<TXT
Режим «Сделай»: ты сам правишь презентацию с помощью инструментов.
Это главное правило: не объясняй пользователю, как сделать вручную, — сделай сам. Если раньше в разговоре
ты был в режиме «Совет» и писал, что не можешь ничего менять, — это больше не так: сейчас режим «Сделай».
Каждое сообщение пользователя начинается с пометки текущего режима — верь ей.
Порядок работы:
1. Пойми задачу. Если она неоднозначна и ошибка дорого стоит — задай один уточняющий вопрос.
2. Посмотри презентацию: состояние приложено к сообщению; текст — read_slides; фигуры и их id — slide_details.
3. Основные команды:
   - слайды: slide.new {layout?, title?, body? (пункты через \\n), at?}; slide.duplicate {index?}; slide.delete {index?};
     slide.move {from, to}; slide.go {index}; slide.layout {layout}; slide.hide {index?, hidden?}; несколько слайдов сразу — add_slides;
     из текста-структуры: slide.fromOutline {text, markdown: true} («# Заголовок» — слайд, «- пункт» — пункт);
   - текст фигуры целиком: text.set {id, text} (абзацы через \\n); заметки докладчика: slide.notes {text, index?};
     найти и заменить во всей презентации: text.replace {text, with};
   - выделить фигуру: selection.select {id}; затем шрифт: format.font {family}, format.size {size}, format.bold {}, format.color {color: "#RRGGBB"};
     абзац: format.align {align: "left|center|right|justify"}, format.bullets {on}, format.numbering {}, format.lineSpacing;
   - вставка: insert.textBox {rect: [x,y,w,h], text}; shape.insert {preset, rect, text?}; insert.table {rows, cols}; insert.chart; insert.smartArt;
   - дизайн: design.theme {name} (список — design.themes), design.variant {index}, design.slideSize {preset: "widescreen"|"standard"};
   - переходы: transition.set {kind, duration?, index?}; ко всем слайдам — transition.applyAll {};
   - анимация: сначала выдели фигуру, затем animation.add {effect, start?: "onClick"|"withPrevious"|"afterPrevious"}.
   Если не знаешь id команды или её параметры — find_commands.
4. После заметных изменений проверь результат (get_state, slide_details или look_at_screen) и коротко скажи, что сделал и как отменить (Ctrl+Z).
Правила безопасности:
- Не создавай новую презентацию, не открывай и не закрывай файлы, не удаляй слайды и большие куски текста, если пользователь прямо об этом не просил.
- Делай не больше 12 действий подряд; если задача больше — сделай часть и спроси, продолжать ли.
- Если команда вернула ошибку — не повторяй вслепую: прочитай ошибку и params_doc, поправь параметры или объясни, что мешает.
TXT;
    }

    return $common . "\n\n" . <<<TXT
Режим «Совет»: ты только объясняешь и подсказываешь, ничего не меняешь сам.
Можно искать команды (find_commands), читать презентацию (read_slides, get_state, slide_details) и смотреть на экран (look_at_screen).
Объясняй, куда нажать и что выставить, с конкретными значениями. Можешь пересказать презентацию, предложить структуру, проверить текст слайдов.
Если пользователь просит изменить презентацию за него — скажи, что для этого нужно переключиться в режим «Сделай» (кнопка вверху панели).
TXT;
}

/**
 * Каталог инструментов текстом — для системной подсказки. Некоторые шлюзы (например, vLLM без
 * парсера вызовов) не передают модели поле tools; тогда модель узнаёт о наших инструментах
 * только отсюда и пишет вызовы текстом.
 */
function studio_tool_catalog(array $tools): string
{
    $lines = [];
    foreach ($tools as $t) {
        $f = $t['function'] ?? [];
        $name = (string)($f['name'] ?? '');
        if ($name === '') {
            continue;
        }
        $params = [];
        foreach (($f['parameters']['properties'] ?? []) as $p => $spec) {
            $params[] = $p . ': ' . ($spec['type'] ?? 'string');
        }
        $lines[] = '- ' . $name . '(' . implode(', ', $params) . ') — ' . (string)($f['description'] ?? '');
    }
    return implode("\n", $lines);
}

/**
 * Вызовы инструментов, которые модель написала текстом, а не штатным полем tool_calls:
 *   <tool_call><function=ИМЯ><parameter=КЛЮЧ>ЗНАЧЕНИЕ</parameter>…</function></tool_call>  (Qwen)
 *   <tool_call>{"name": "ИМЯ", "arguments": {…}}</tool_call>                             (Hermes)
 * Возвращает [вызовы, текст без этих блоков].
 */
function studio_text_tool_calls(string $content): array
{
    $calls = [];
    $value = static function (string $raw) {
        $raw = trim($raw);
        $j = json_decode($raw, true);
        return ($j !== null || $raw === 'null') ? $j : $raw;
    };
    // Qwen XML: с обёрткой <tool_call> или без неё. Прокси с vLLM иногда «съедает» начало
    // вызова («<tool_call>\n<»), и приходит «function=run_command>…» — это тоже понимаем,
    // как и вызов без закрывающих тегов в конце ответа.
    if (preg_match_all('~(?:<tool_call>\s*)?<?function=([A-Za-z0-9_.\-]+)>(.*?)(?:</function>(?:\s*</tool_call>)?|(?=<?function=[A-Za-z0-9_.\-]+>)|\z)~s', $content, $m, PREG_SET_ORDER)) {
        foreach ($m as $one) {
            $args = [];
            if (preg_match_all('~<?parameter=([A-Za-z0-9_.\-]+)>(.*?)(?:</parameter>|(?=<?parameter=[A-Za-z0-9_.\-]+>)|(?=</function>)|\z)~s', $one[2], $pm, PREG_SET_ORDER)) {
                foreach ($pm as $p) {
                    $args[$p[1]] = $value($p[2]);
                }
            }
            $calls[] = ['function' => ['name' => $one[1], 'arguments' => $args ?: new stdClass()]];
            $content = str_replace($one[0], '', $content);
        }
    }
    // Hermes: JSON внутри <tool_call>.
    if (preg_match_all('~<tool_call>\s*(\{.*?\})\s*</tool_call>~s', $content, $m, PREG_SET_ORDER)) {
        foreach ($m as $one) {
            $j = json_decode($one[1], true);
            if (is_array($j) && isset($j['name'])) {
                $args = $j['arguments'] ?? ($j['parameters'] ?? []);
                if (is_string($args)) {
                    $args = json_decode($args, true) ?: [];
                }
                $calls[] = ['function' => ['name' => (string)$j['name'], 'arguments' => $args ?: new stdClass()]];
                $content = str_replace($one[0], '', $content);
            }
        }
    }
    // Рассуждения модели в ответ не нужны.
    $content = preg_replace('~<think>.*?</think>~s', '', $content) ?? $content;
    $content = trim(str_replace(['<tool_call>', '</tool_call>'], '', $content));
    return [$calls, $content];
}
