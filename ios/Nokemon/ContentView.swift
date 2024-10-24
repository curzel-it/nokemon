//
//  ContentView.swift
//  Nokemon
//
//  Created by Federico Curzel on 23/10/24.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
        }
        .padding()
        .onAppear {
            
            test_integration()
            
            initialize_config(
                "en",
                folderContaining(name: "1001", extension: "json", folder: "data"),
                filePath(name: "species", extension: "json", folder: "data"),
                filePath(name: "inventory", extension: "json", folder: "data"),
                saveFilePath(),
                folderContaining(name: "en", extension: "stringx", folder: "lang")
            )
            
            initialize_game(false)
            window_size_changed(400, 400, 1, 1, 1)
            update_game(0.1)
            
            let renderableItems = fetchRenderableItems()
            for item in renderableItems {
                print("Sprite Sheet ID: \(item.sprite_sheet_id)")
                print("Texture Rect: (x: \(item.texture_rect.x), y: \(item.texture_rect.y), width: \(item.texture_rect.width), height: \(item.texture_rect.height))")
                print("Offset: (x: \(item.offset.x), y: \(item.offset.y))")
                print("Frame: (x: \(item.frame.x), y: \(item.frame.y), width: \(item.frame.width), height: \(item.frame.height))")
            }
        }
    }
}

@frozen
public struct IntRect {
    public var x: Float
    public var y: Float
    public var width: Float
    public var height: Float
}

@frozen
public struct Vector2d {
    public var x: Float
    public var y: Float
}

@frozen
public struct RenderableItem {
    public var sprite_sheet_id: UInt32
    public var texture_rect: IntRect
    public var offset: Vector2d
    public var frame: IntRect
}

@_silgen_name("renderables")
func renderables(_ length: UnsafeMutablePointer<size_t>?) -> UnsafeMutablePointer<RenderableItem>?

@_silgen_name("free_renderables")
func free_renderables(_ ptr: UnsafeMutablePointer<RenderableItem>?, _ length: size_t)

func fetchRenderableItems() -> [RenderableItem] {
    var length: size_t = 0

    guard let ptr = renderables(&length) else {
        print("Failed to fetch renderables")
        return []
    }

    let buffer = UnsafeBufferPointer<RenderableItem>(start: ptr, count: length)
    let items = Array(buffer)

    free_renderables(ptr, length)

    return items
}

func filePath(name: String, extension ext: String, folder: String) -> String {
    Bundle.main.url(forResource: name, withExtension: ext, subdirectory: folder)?
        .absoluteString
        .replacingOccurrences(of: "file:///", with: "/") ?? "iOS file not found \(folder)/\(name).\(ext)"
}

func folderContaining(name: String, extension ext: String, folder: String) -> String {
    filePath(name: name, extension: ext, folder: folder)
        .replacingOccurrences(of: "/\(name).\(ext)", with: "")
}

func saveFilePath() -> String {
    let fileManager = FileManager.default
    let documentsDirectory = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first!
    let saveFileURL = documentsDirectory.appendingPathComponent("save.json")
    
    if !fileManager.fileExists(atPath: saveFileURL.path) {
        let defaultContents = "{\"always\": 1}".data(using: .utf8)
        fileManager.createFile(atPath: saveFileURL.path, contents: defaultContents, attributes: nil)
    }
    
    return saveFileURL.path
}
