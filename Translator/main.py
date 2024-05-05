from googletrans import Translator
import argparse

def read_file(filename):
    try:
        with open(filename, 'r', encoding='utf-8') as file:
            content = file.read()
        return content
    except FileNotFoundError:
        print("File not found:", filename)
        return None

def write_to_file(filename, content):
    try:
        with open(filename, 'w', encoding='utf-8') as file:
            file.write(content)
        print("Write to file '{}' successful.".format(filename))
    except Exception as e:
        print("Error:", e)


def main():
    # 创建 ArgumentParser 对象
    parser = argparse.ArgumentParser(description='My script description.')

    # 添加选项
    parser.add_argument('-m', '--mode', metavar='MODE', choices=['file', 'text'],
                        help='Specify the mode: file or text.')
    parser.add_argument('-f', '--file', metavar='FILENAME', help='Specify the input file.')
    parser.add_argument('-o', '--output', metavar='FILENAME', help='Specify the output file.')
    parser.add_argument('-t', '--text', metavar='TEXT', help='Specify the text needed translated.')
    parser.add_argument('-s', '--src', metavar='src_lang', help='The source language, default: auto.')
    parser.add_argument('-d', '--dest', metavar='dest_lang', help='The dest language, default: zh-cn.')

    # 解析命令行参数
    args = parser.parse_args()


    
    src = 'auto' if args.src == None else args.src
    dest = 'zh-cn' if args.dest == None else args.dest
    
    # 创建一个 Translator 对象
    translator = Translator()

    if args.mode == 'file':
        text = read_file(args.file)
    else: 
        text = args.text

    translated_text = translator.translate(text, src=src, dest=dest)

    if args.output == None:
        print(translated_text.text)
    else :
        write_to_file(args.output, translated_text.text)    

if __name__ == '__main__':
    main()
