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
            initialize_game(false)
            window_size_changed(400, 400, 1, 1, 1)
            update_game(0.1)
            
            var itemsToRenderCount: UnsafeMutablePointer<Int>
            let itemsToRender = renderables(itemsToRenderCount)
            
            print(itemsToRenderCount)
            print(itemsToRender)
        }
    }
}
