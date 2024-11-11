using System;
using UnityEngine;
using UnityEngine.UI;

namespace Minesweeper
{
    public class Console : MonoBehaviour
    {
        public static Console Instance { get; private set; }

        private Text _text;

        private void Awake()
        {
            Instance = this;

            _text = GetComponent<Text>();

            _text.text = string.Empty;
        }

        public void Log(string message)
        {
            _text.text += message + "\n";
            Debug.Log(message);
        }
    }
}